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
    pub text: &'a str,
    pub cursor: usize,
}

impl<'a> Input<'a> {
    pub fn new(text: &'a str) -> Self {
        Input { text, cursor: 0 }
    }

    pub fn try_match(&mut self, regex: &Regex) -> Option<Match<'a>> {
        let text = &self.text[self.cursor..self.text.len()];
        regex.find(text)
    }
    pub fn try_capture(&self, regex: &Regex) -> Option<Captures<'a>> {
        let text = &self.text[self.cursor..self.text.len()];
        regex.captures(text)
    }

    pub fn move_forward(&mut self, amount: usize) -> bool {
        self.cursor += amount;
        if self.cursor > self.text.len() {
            self.cursor = self.text.len();
            false
        } else {
            true
        }
    }
    pub fn move_backward(&mut self, amount: usize) -> bool {
        match self.cursor.checked_sub(amount) {
            Some(cursor) => {
                self.cursor = cursor;
                true
            }
            None => false,
        }
    }
    pub fn skip_forward(&mut self, regex: &Regex) -> usize {
        let text = &self.text[self.cursor..self.text.len()];
        let chars = match regex.find(text) {
            Some(m) => m.end(),
            None => 0,
        };
        self.move_forward(chars);
        chars
    }
    pub fn skip_whitespace(&mut self) -> usize {
        lazy_static! {
            static ref WHITESPACE: Regex = Regex::new(r"[ \t]+").unwrap();
        }
        self.skip_forward(&WHITESPACE)
    }
    pub fn skip_whitespace_newline(&mut self) -> usize {
        lazy_static! {
            static ref WHITESPACE_NEWLINE: Regex = Regex::new(r"[ \t\r\n]+").unwrap();
        }
        self.skip_forward(&WHITESPACE_NEWLINE)
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
