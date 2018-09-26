//! Contains the types and traits needed for parsing.
use super::*;
use regex::{Captures, Match, Regex};

/// Input that can be parsed.
///
/// # Implementing a parser
///
/// ```
/// extern crate rust_orgmode;
/// extern crate regex;
///
/// use regex::{Regex, Captures};
/// use rust_orgmode::types::SharedBehaviorData;
/// use rust_orgmode::types::parsing::{self, Input, ParseError};
///
/// let mut input = Input::new("Parse 2^5.");
///
/// let regex = Regex::new(r"(?m)\AParse (?P<number1>\d+)\^(?P<number2>\d+)").unwrap();
///
/// // collects the data from the capture groups of the regex
/// fn collect_data(input: &mut Input, captures: Captures) -> parsing::Result<(u32, u32)> {
///     let number1_match = captures.name("number1").ok_or(ParseError)?;
///     let number1 = number1_match.as_str().parse().map_err(|_| ParseError)?;
///     let number2_match = captures.name("number2").ok_or(ParseError)?;
///     let number2 = number2_match.as_str().parse().map_err(|_| ParseError)?;
///
///     // advance cursor so this will not be parsed again
///     input.move_forward(number2_match.end());
///
///     Ok((number1, number2))
/// }
///
/// // creates the actual result type
/// fn construct_result((number1, number2): (u32, u32), sbd: SharedBehaviorData) -> parsing::Result<u32> {
///     Ok(number1.pow(number2))
/// }
///
/// let result = input.do_parse(&regex, collect_data, construct_result);
///
/// assert_eq!(result, Ok(32u32));
/// ```
///
/// Use `(?m)` in your regex so you can use `^` and `$` for start/end of line. You can then use `\A` to anchor to
/// the beginning of the input. That will prevent you from acidentally skipping characters and
/// instead fail to parse.
///
/// `collect_data` and `construct_result` is split in two so  can [`do_parse`] take care of creating
/// the [`SharedBehaviorData`].
///
/// See [`do_parse`] for more info.
///
/// [`do_parse`]: `Input::do_parse`
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
    pub fn try_captures(&self, regex: &Regex) -> Option<Captures<'a>> {
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

    /// Helper function to make parsing easier.
    ///
    /// The [`Regex`] is used to capture groups which are then given to `collect_data`. The
    /// `collect_data` function retrieves the needed capture groups and converts them to a usable
    /// format. This function also needs to move the cursor of the input appropriately. If collecting
    /// the data fails the function has to return an error. This error is then returned from this
    /// function.
    ///
    /// The `construct_result` function receives the result of `collect_data` (which is usually a single
    /// value or a tuple) and the [`SharedBehaviorData`] for the parsed object/element and needs to
    /// construct the object/element. This function can't fail so the data has to be validated in
    /// `collect_data`.
    pub fn do_parse<T, F1, F2, R>(
        &mut self,
        regex: &Regex,
        collect_data: F1,
        construct_result: F2,
    ) -> Result<R>
    where
        F1: FnOnce(&mut Input, Captures) -> Result<T>,
        F2: FnOnce(T, SharedBehaviorData) -> Result<R>,
    {
        let start = self.cursor;
        let captures = self.try_captures(regex).ok_or(ParseError)?;
        let value = collect_data(self, captures)?;
        let end = self.cursor - 1;
        let post_blank = self.skip_whitespace();

        let span = Span::new(start, end);
        let shared_behavior_data = SharedBehaviorData { span, post_blank };

        construct_result(value, shared_behavior_data)
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
