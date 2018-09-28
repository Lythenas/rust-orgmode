//! Contains the types and traits needed for parsing.
use super::*;
use regex::{Captures, Match, Regex};
use std::ops::Deref;
use std::slice::SliceIndex;

#[derive(Debug, Clone)]
pub struct Context {
    cursor: Cursor,
}

impl Context {
    pub fn move_cursor_forward(&mut self, amount: usize) {
        self.cursor.forward(amount);
    }
}

#[derive(Debug)]
pub struct Parser {
    input: Input,
    cursor: Cursor,
}

impl From<Input> for Parser {
    fn from(input: Input) -> Parser {
        let input_len = input.len();
        Parser {
            input,
            cursor: Cursor {
                pos: 0,
                input_len,
            },
        }
    }
}

impl Parser {
    fn create_context(&self) -> Context {
        Context {
            cursor: self.cursor.clone(),
        }
    }

    /// Helper function to make parsing easier.
    ///
    /// The [`Regex`] is used to capture groups. Use `(?m)` in your regex so you can use `^` and `$`
    /// for start/end of line. You can then use `\A` to anchor to the beginning of the input. That
    /// will prevent you from acidentally skipping characters and instead fail to parse.
    ///
    /// `collect_data` takes these [`Captures`] and uses them to extract the needed data and move
    /// the cursor of the input forward (after the parsed input). If collecting the data fails this
    /// function returns an error. If collecting the data succeeds the function returns the collected
    /// data `T` (this is usually a tuple).
    ///
    /// `construct_result` takes the collected data `T` and creates the final [`Object`] or
    /// [`Element`] struct `R`. For this it also receives the [`SharedBehaviorData`] that it needs to
    /// construct the type. This function can also fail and return an error. However it is probably
    /// easier to directly fail in `collect_data`.
    ///
    /// The errors returned from `collect_data` and `construct_result` can be any type that can be
    /// converted to [`ParseError`]. If one of the functions return an error the [`Cursor`] of the
    /// [`Input`] will be reset.
    pub fn parse_object<T, R, E1, E2>(
        &mut self,
        regex: &Regex,
        collect_data: impl FnOnce(&mut Context, &Captures) -> Result<T, E1>,
        construct_result: impl FnOnce(T, SharedBehaviorData) -> Result<R, E2>,
    ) -> Result<R, ParseError>
    where
        ParseError: From<E1> + From<E2>,
    {
        let start = self.cursor.pos();
        let captures = self
            .input
            .try_captures(regex, start..)
            .ok_or(ParseError)?;

        let mut context = self.create_context();

        let value = match collect_data(&mut context, &captures) {
            Ok(value) => value,
            Err(err) => {
                return Err(ParseError::from(err));
            }
        };

        let end = context.cursor.pos();
        let post_blank = self.input.count_whitespace(end..);
        context.cursor.forward(post_blank);

        let span = Span::new(start, end - 1);
        let shared_behavior_data = SharedBehaviorData { span, post_blank };

        let result = construct_result(value, shared_behavior_data)?;

        self.cursor = context.cursor;

        Ok(result)
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor.pos()
    }
}

/// Input that can be parsed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Input {
    text: String,
}

impl Deref for Input {
    type Target = str;
    fn deref(&self) -> &str {
        &self.text
    }
}

impl Input {
    pub fn new(text: impl ToString) -> Self {
        Input {
            text: text.to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn try_match<I>(&self, regex: &Regex, index: I) -> Option<Match>
    where
        I: SliceIndex<str, Output = str>,
    {
        let text = self.text.get(index)?;
        regex.find(text)
    }
    pub fn try_captures<I>(&self, regex: &Regex, index: I) -> Option<Captures>
    where
        I: SliceIndex<str, Output = str>,
    {
        let text = self.text.get(index)?;
        regex.captures(text)
    }

    pub fn count_forward<I>(&self, regex: &Regex, index: I) -> usize
    where
        I: SliceIndex<str, Output = str>,
    {
        let text = match self.text.get(index) {
            Some(text) => text,
            None => return 0,
        };
        regex.find(text).map(|m| m.end()).unwrap_or(0)
    }
    pub fn count_whitespace<I>(&self, index: I) -> usize
    where
        I: SliceIndex<str, Output = str>,
    {
        lazy_static! {
            static ref WHITESPACE: Regex = Regex::new(r"[ \t]+").unwrap();
        }
        self.count_forward(&WHITESPACE, index)
    }
    pub fn count_whitespace_newline<I>(&self, index: I) -> usize
    where
        I: SliceIndex<str, Output = str>,
    {
        lazy_static! {
            static ref WHITESPACE_NEWLINE: Regex = Regex::new(r"[ \t\r\n]+").unwrap();
        }
        self.count_forward(&WHITESPACE_NEWLINE, index)
    }
}

/// Cursor of [`Input`].
///
/// Tracks the current position in the input. Modify the cursor using
/// [`forward`][`Cursor::forward`] and [`backward`][`Cursor::backward`].
///
/// TODO decide if implementing `Clone` is correct. Because the cursor is invalid if it is cloned
/// and the input is dropped.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    pos: usize,
    input_len: usize,
}

impl Cursor {
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn forward(&mut self, amount: usize) -> bool {
        self.pos += amount;
        if self.pos > self.input_len {
            self.pos = self.input_len;
            false
        } else {
            true
        }
    }
    pub fn backward(&mut self, amount: usize) -> bool {
        match self.pos.checked_sub(amount) {
            Some(pos) => {
                self.pos = pos;
                true
            }
            None => false,
        }
    }
}

/// An error which can be returned when parsing a [`Document`] or any of its components.
///
/// [`Document`]: `document::Document`
///
/// TODO improve this probably make this an enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError;

impl From<()> for ParseError {
    fn from(_: ()) -> ParseError {
        ParseError
    }
}

impl From<!> for ParseError {
    fn from(x: !) -> ParseError {
        x
    }
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
}
