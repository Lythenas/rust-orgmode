//! Contains the types and traits needed for parsing.

use crate::types::document::Document;
use crate::types::greater_elements::{Headline, Section};

use pest::iterators::Pair;
#[allow(unused_imports)]
use pest::{self, Parser};

#[derive(Parser)]
#[grammar = "orgmode.pest"]
pub struct OrgModeParser;

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
        let mut rules = pair.into_inner().peekable();

        let preface = match rules.peek() {
            Some(pair) if pair.as_rule() == Rule::preface => {
                Some(parse_preface(rules.next().unwrap().into_inner())?)
            }
            _ => None,
        };

        // Try to parse all headlines and fails at the first Err
        // TODO maybe collect all errors and return them all instead of
        // just the first (using Itertools::partition_map)
        let headlines: Vec<_> = rules
            .skip_while(|pair| pair.as_rule() == Rule::preface)
            .map(|pair| parse_headline(pair.into_inner()))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(Document { preface, headlines });
    }
    // The document rule can't fail. Worst case it is just empty ("SOI ~ EOI").
    unreachable!("document rule can't fail")
}

fn parse_preface<'i, I>(_pairs: I) -> Result<Section, ParseError>
where
    I: Iterator<Item = Pair<'i, Rule>>,
{
    unimplemented!()
}

fn parse_headline<'i, I>(_pairs: I) -> Result<Headline, ParseError>
where
    I: Iterator<Item = Pair<'i, Rule>>,
{
    unimplemented!()
}

#[cfg(test)]
mod tests {
    //use super::*;
}
