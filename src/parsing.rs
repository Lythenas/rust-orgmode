//! Contains the types and traits needed for parsing.

use crate::types::document::Document;
use crate::types::elements::Paragraph;
use crate::types::greater_elements::{Headline, Section};
use crate::types::{ElementSet, SecondaryString, Span, Spanned, StandardSet};

use pest::iterators::Pair;
#[allow(unused_imports)]
use pest::{self, Parser};

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
            .skip_while(|pair| pair.as_rule() == Rule::preface)
            .map(parse_headline)
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(Document { preface, headlines });
    }
    // The document rule can't fail. Worst case it is just empty ("SOI ~ EOI").
    unreachable!("document rule can't fail")
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

fn parse_headline<'i>(_pair: Pair<'i, Rule>) -> Result<Headline, ParseError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    //use super::*;
}
