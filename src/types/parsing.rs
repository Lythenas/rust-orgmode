//! Contains the types and traits needed for parsing.
use regex::{Captures, Match, Regex};

/// Input that can be parsed.
///
/// # Usage
///
/// ```
/// let input = Input::new("Some input read from file.");
/// // regex match / etc
/// // advance cursor
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Input<'a> {
    pub input: &'a str,
    pub cursor: usize,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a str) -> Self {
        Input { input, cursor: 0 }
    }

    pub fn try_match(&mut self, regex: &Regex) -> Option<Match<'a>> {
        let input = &self.input[self.cursor..self.input.len()];
        let re_match = regex.find(input);

        if let Some(ref m) = &re_match {
            self.cursor += m.end();
        };

        re_match
    }
    pub fn try_capture(&mut self, regex: &Regex) -> Option<Captures<'a>> {
        let re_captures = regex.captures(&self.input[self.cursor..self.input.len()]);

        if let Some(ref c) = &re_captures.iter().last() {
            if let Some(ref m) = &c.iter().last().unwrap() {
                self.cursor += m.end();
            }
        };

        re_captures
    }
    pub fn backup_cursor(&mut self, amount: usize) -> bool {
        match self.cursor.checked_sub(amount) {
            Some(cursor) => {
                self.cursor = cursor;
                true
            }
            None => false,
        }
    }
}

/// Result of trying to parse a [`Input`].
pub type Result<T> = ::std::result::Result<T, ParseError>;

// TODO improve this probably make this an enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError;

pub trait Parse: Sized {
    fn parse(input: &mut Input) -> Result<Self>;
}

/// Convenience trait to implement [`Parse`] for blocks.
pub trait ParseBlock: Sized {}

impl<T: ParseBlock> Parse for T {
    fn parse(_input: &mut Input) -> Result<Self> {
        unimplemented!()
    }
}
