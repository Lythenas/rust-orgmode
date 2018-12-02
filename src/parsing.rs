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
        return Document::try_from_iter(pair.into_inner());
    }
    // The document rule can't fail. Worst case it is just "SOI ~ EOI".
    unreachable!("document rule can't fail")
}

trait TryFromIterator<A> {
    type Result;
    type Error;
    fn try_from_iter<T>(iter: T) -> Result<Self::Result, Self::Error>
    where
        T: IntoIterator<Item = A>;
}

impl<'i> TryFromIterator<Pair<'i, Rule>> for Document {
    type Result = Document;
    type Error = ParseError;
    fn try_from_iter<T>(iter: T) -> Result<Self::Result, Self::Error>
    where
        T: IntoIterator<Item = Pair<'i, Rule>>,
    {
        let mut preface = None;
        let mut headlines = Vec::new();

        for pair in iter {
            match pair.as_rule() {
                Rule::preface => {
                    if preface.is_none() {
                        preface = Some(parse_preface(pair.into_inner())?);
                    } else {
                        // TODO check if this can happen
                        return Err(ParseError::StructuralError("found two prefaces"));
                    }
                }
                Rule::headline => headlines.push(parse_headline(pair.into_inner())?),
                _ => unreachable!("document rule can only contain preface and headlines"),
            }
        }

        Ok(Document { preface, headlines })
    }
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
