//! Contains the types and traits needed for parsing.

use crate::types::document::Document;
use crate::types::elements::Paragraph;
use crate::types::greater_elements::{Headline, HeadlineContentSet, Section};
use crate::types::{
    ElementSet, SecondaryString, Span, Spanned, StandardSet, StandardSetNoLineBreak,
};

use pest::iterators::Pair;
#[allow(unused_imports)]
use pest::{self, Parser};

use itertools::Itertools;

use std::iter::Peekable;

#[derive(Parser)]
#[grammar = "orgmode.pest"]
pub struct OrgModeParser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ParseError {
    LexError(pest::error::Error<Rule>),
    StructuralError(&'static str), // TODO define more specifically and maybe rename
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        ParseError::LexError(error)
    }
}

/// Helper function to create predicates to filter for or skip the specified rule.
fn is_rule<'i>(rule: Rule) -> impl Fn(&Pair<'i, Rule>) -> bool {
    move |pair| pair.as_rule() == rule
}

pub fn parse_document(s: &str) -> Result<Document, ParseError> {
    if let Some(pair) = OrgModeParser::parse(Rule::document, &s)?.next() {
        assert_eq!(pair.as_rule(), Rule::document);

        let mut rules = pair.into_inner().peekable();

        let preface = match rules.peek() {
            Some(pair) if pair.as_rule() == Rule::preface => {
                Some(parse_preface(rules.next().unwrap())?)
            }
            _ => None,
        };

        // Try to parse all headlines and fails at the first Err
        // TODO maybe collect all errors and return them all instead of
        // just the first (using Itertools::partition_map)
        let headlines: Vec<_> = rules
            .by_ref()
            .skip_while(|pair| pair.as_rule() == Rule::preface)
            .peekable()
            .peeking_take_while(is_rule(Rule::headline))
            .map(parse_headline)
            .collect::<Result<Vec<_>, _>>()?;

        // TODO The last rule should be EOI, but assert fails
        // assert_eq!(rules.next().map(|p| p.as_rule()), Some(Rule::EOI));

        let nested_headlines = nest_headlines(&mut headlines.into_iter().peekable());

        return Ok(Document {
            preface,
            headlines: nested_headlines,
        });
    }
    // The document rule can't fail. Worst case it is just empty ("SOI ~ EOI").
    unreachable!("document rule can't fail")
}

/// Nests headlines correctly.
///
/// This function calls itself recursively and returns a list of modified
/// headlines of the lowest level with higher level headlines nested in them
/// correctly.
///
/// The iterator is peekable because the recursive calls skip over all the nested
/// headlines and return to the lower level headlines. Without peekable we would skip
/// headlines.
fn nest_headlines(headlines: &mut Peekable<impl Iterator<Item = Headline>>) -> Vec<Headline> {
    let mut collector = Vec::new();
    collector.push(match headlines.next() {
        None => return collector,
        Some(h) => h,
    });

    while let Some(headline) = headlines.peek() {
        let level = collector.last().unwrap().level;
        let current = collector.last_mut().unwrap();
        if headline.level > level {
            // nest the headline
            current.push_content(
                nest_headlines(headlines)
                    .into_iter()
                    .map(Box::new)
                    .map(HeadlineContentSet::Headline),
            )
        } else if headline.level < level {
            // return to higher headline
            return collector;
        } else {
            // insert the headline at the same level
            collector.push(headlines.next().unwrap());
        }
    }

    collector
}

fn parse_preface<'i>(pair: Pair<'i, Rule>) -> Result<Section, ParseError> {
    assert_eq!(pair.as_rule(), Rule::preface);
    let pair = pair.into_inner().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::section);

    // TODO extend to parse grater elements, elements and objects once they are
    // in the grammar
    let span = pair.as_span().into();
    let value: Vec<_> = pair
        .into_inner()
        .map(parse_paragraph)
        .map(|result| result.map(|paragraph| ElementSet::from(paragraph)))
        .collect::<Result<Vec<_>, _>>()?;
    let content = Spanned::with_span(value, span);

    Ok(Section::new(content))
}

fn parse_paragraph<'i>(pair: Pair<'i, Rule>) -> Result<Paragraph, ParseError> {
    assert_eq!(pair.as_rule(), Rule::paragraph);

    let _span: Span = pair.as_span().into();
    let value = pair.as_str().to_string();

    Ok(Paragraph::new(SecondaryString::with_one(
        StandardSet::RawString(value),
    )))
}

fn parse_headline<'i>(pair: Pair<'i, Rule>) -> Result<Headline, ParseError> {
    assert_eq!(pair.as_rule(), Rule::headline);

    let _span: Span = pair.as_span().into();

    let mut inner = pair.into_inner().peekable();
    let affiliated_keywords = inner
        .by_ref()
        .peeking_take_while(is_rule(Rule::affiliated_keywords))
        .take(1)
        .map(|_p| unimplemented!()) // TODO parse_affiliated_keywords
        .next();
    let stars = inner
        .by_ref()
        .take(1)
        .filter(is_rule(Rule::stars))
        .map(|p| p.as_str().len())
        .next()
        .unwrap(); // grammar guarantees at least one star
    let stars = if stars <= u32::max_value() as usize {
        stars as u32
    } else {
        return Err(ParseError::StructuralError(
            "to many stars in headline (more than 2^32-1)",
        ));
    };
    let title = inner
        .by_ref()
        .skip_while(is_rule(Rule::BLANK))
        .take(1)
        .filter(is_rule(Rule::title))
        .map(parse_secondary_string_no_line_break)
        .next()
        .unwrap()?; // grammar guarantees a (empty) title
    let planning = inner
        .by_ref()
        //.skip_while(is_rule(Rule::NEWLINE))
        .peeking_take_while(is_rule(Rule::planning))
        .take(1)
        .map(|_p| unimplemented!())
        .next();
    let section = inner
        .by_ref()
        .peeking_take_while(is_rule(Rule::section))
        .take(1)
        .map(|_p| unimplemented!())
        .next();

    // TODO figure out the correct span (probably directly when finding the section)
    let content = section.map(Spanned::new);

    Ok(Headline {
        affiliated_keywords,
        content,
        level: stars,
        todo_keyword: None, // TODO
        priority: None,     // TODO
        title: Some(title), // TODO better error handling for title
        tags: Vec::new(),   // TODO
        planning,
        property_drawer: None,
    })
}

fn parse_secondary_string_no_line_break<'i>(
    pair: Pair<'i, Rule>,
) -> Result<SecondaryString<StandardSetNoLineBreak>, ParseError> {
    Ok(SecondaryString::with_one(
        StandardSetNoLineBreak::RawString(pair.as_str().to_string()),
    ))
}

#[cfg(test)]
mod tests {
    //use super::*;
}
