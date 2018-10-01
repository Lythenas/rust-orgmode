//! Contains the types and traits needed for parsing.
use crate::types::affiliated_keywords::AffiliatedKeywords;
use crate::types::*;
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
pub struct Parser<'a> {
    input: Input<'a>,
    cursor: Cursor,
}

impl<'a> From<Input<'a>> for Parser<'a> {
    fn from(input: Input<'a>) -> Parser<'a> {
        let limit = input.len();
        Parser {
            input,
            cursor: Cursor { pos: 0, limit },
        }
    }
}

impl Parser<'_> {
    fn clone_with_cursor(&self, cursor: Cursor) -> Result<Parser<'_>, ParseError> {
        if cursor.limit > self.cursor.limit {
            Err(ParseError {
                kind: ParseErrorKind::Internal("tried to set higher limit"),
            })
        } else {
            Ok(Parser {
                input: self.input.clone(),
                cursor,
            })
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
        collect_data: impl FnOnce(&mut Context, &Captures<'_>) -> Result<T, E1>,
        construct_result: impl FnOnce(T, SharedBehaviorData) -> Result<R, E2>,
    ) -> Result<R, ParseError>
    where
        ParseError: From<E1> + From<E2>,
    {
        let start = self.cursor.pos();
        let captures = self.input.try_captures(regex, start..).ok_or(ParseError {
            kind: ParseErrorKind::CantFindObject,
        })?;

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
        let shared_behavior_data = SharedBehaviorData::new(span, post_blank);

        let result = construct_result(value, shared_behavior_data)?;

        self.cursor = context.cursor;

        Ok(result)
    }

    // TODO find a way to reduce code duplicate
    pub fn parse_block_with_dynamic_end<'a, T, R, E1, E2, S>(
        &mut self,
        start_re: &Regex,
        get_end_re: impl FnOnce(&Context, &Captures<'_>) -> Regex,
        collect_data: impl FnOnce(&mut Context, &Captures<'_>) -> Result<T, E1>,
        construct_result: impl FnOnce(
            T,
            SharedBehaviorData,
            Spanned<AffiliatedKeywords>,
            ContentData<S>,
        ) -> Result<R, E2>,
    ) -> Result<R, ParseError>
    where
        ParseError: From<E1> + From<E2>,
        T: std::fmt::Debug,
        S: std::fmt::Debug + Parse,
        R: std::fmt::Debug,
    {
        // capture start line
        let start = self.cursor.pos();
        let captures = self
            .input
            .try_captures(start_re, start..)
            .ok_or(ParseError {
                kind: ParseErrorKind::CantFindStartOfBlock,
            })?;
        let end_of_start = captures.get(0).unwrap().end();

        let mut context = self.create_context();

        // find end line
        // start search after the start line
        let end_re = get_end_re(&context, &captures);
        let end_match = self
            .input
            .try_match(&end_re, end_of_start..)
            .ok_or(ParseError {
                kind: ParseErrorKind::CantFindEndOfBlock,
            })?;

        // parse start line
        // this also moves the cursor to after the content
        let value = match collect_data(&mut context, &captures) {
            Ok(value) => value,
            Err(err) => {
                return Err(ParseError::from(err));
            }
        };

        // TODO parse content
        let content_start = end_of_start;
        let content_end = content_start + end_match.start();
        let mut content_parser = self.clone_with_cursor(Cursor {
            pos: content_start,
            limit: content_end,
        })?;
        let content_data = ContentData::parse(&mut content_parser)?;

        context.cursor.forward(end_match.end() + 1);
        let end = context.cursor.pos();
        let span = Span::new(start, end);

        // count whitespace after the end
        let post_blank = self.input.count_whitespace_newline(end..);
        context.cursor.forward(post_blank);

        let shared_behavior_data = SharedBehaviorData::new(span, post_blank);

        // TODO get affiliated keywords from somewhere
        // affiliated keywords are parsed before the element is parsed
        let affiliated_keywords_data = Spanned::new(Span::new(0, 0), AffiliatedKeywords::default());

        let result = construct_result(
            value,
            shared_behavior_data,
            affiliated_keywords_data,
            content_data,
        )?;

        self.cursor = context.cursor;

        Ok(result)
    }

    pub fn parse_block<'a, T, R, E1, E2, S>(
        &mut self,
        start_re: &Regex,
        end_re: &Regex,
        collect_data: impl FnOnce(&mut Context, &Captures<'_>) -> Result<T, E1>,
        construct_result: impl FnOnce(
            T,
            SharedBehaviorData,
            Spanned<AffiliatedKeywords>,
            ContentData<S>,
        ) -> Result<R, E2>,
    ) -> Result<R, ParseError>
    where
        ParseError: From<E1> + From<E2>,
        T: std::fmt::Debug,
        S: std::fmt::Debug + Parse,
        R: std::fmt::Debug,
    {
        // capture start line
        let start = self.cursor.pos();
        let captures = self
            .input
            .try_captures(start_re, start..)
            .ok_or(ParseError {
                kind: ParseErrorKind::CantFindStartOfBlock,
            })?;
        let end_of_start = captures.get(0).unwrap().end();

        let mut context = self.create_context();

        // find end line
        // start search after the start line
        let end_match = self
            .input
            .try_match(end_re, end_of_start..)
            .ok_or(ParseError {
                kind: ParseErrorKind::CantFindEndOfBlock,
            })?;

        // parse start line
        // this also moves the cursor to after the content
        let value = match collect_data(&mut context, &captures) {
            Ok(value) => value,
            Err(err) => {
                return Err(ParseError::from(err));
            }
        };

        // TODO parse content
        let content_start = end_of_start;
        let content_end = content_start + end_match.start();
        let mut content_parser = self.clone_with_cursor(Cursor {
            pos: content_start,
            limit: content_end,
        })?;
        let content_data = ContentData::parse(&mut content_parser)?;

        context.cursor.forward(end_match.end() + 1);
        let end = context.cursor.pos();
        let span = Span::new(start, end);

        // count whitespace after the end
        let post_blank = self.input.count_whitespace_newline(end..);
        context.cursor.forward(post_blank);

        let shared_behavior_data = SharedBehaviorData::new(span, post_blank);

        // TODO get affiliated keywords from somewhere
        // affiliated keywords are parsed before the element is parsed
        let affiliated_keywords_data = Spanned::new(Span::new(0, 0), AffiliatedKeywords::default());

        let result = construct_result(
            value,
            shared_behavior_data,
            affiliated_keywords_data,
            content_data,
        )?;

        self.cursor = context.cursor;

        Ok(result)
    }

    fn create_context(&self) -> Context {
        Context {
            cursor: self.cursor.clone(),
        }
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor.pos()
    }
    pub fn has_content_left_to_parse(&self) -> bool {
        self.cursor.pos() < self.cursor.limit()
    }
}

/// Input that can be parsed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Input<'a> {
    text: &'a str,
}

impl Deref for Input<'_> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.text
    }
}

impl Input<'_> {
    pub fn new<'a>(text: &'a str) -> Input<'a> {
        Input { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn try_match<I>(&self, regex: &Regex, index: I) -> Option<Match<'_>>
    where
        I: SliceIndex<str, Output = str>,
    {
        let text = self.text.get(index)?;
        regex.find(text)
    }
    pub fn try_captures<I>(&self, regex: &Regex, index: I) -> Option<Captures<'_>>
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
    limit: usize,
}

impl Cursor {
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn forward(&mut self, amount: usize) -> bool {
        self.pos += amount;
        if self.pos > self.limit {
            self.pos = self.limit;
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
/// TODO improve this
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError {
    kind: ParseErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseErrorKind {
    Unknown,
    Internal(&'static str),
    CantFindStartOfBlock,
    CantFindEndOfBlock,
    CantFindObject,
}

impl From<()> for ParseError {
    fn from(_: ()) -> ParseError {
        ParseError {
            kind: ParseErrorKind::Unknown,
        }
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

impl Parse for Spanned<String> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r".+\n*").unwrap();
        }

        fn collect_data(_: &mut Context, captures: &Captures) -> Result<String, ParseError> {
            Ok(captures.get(0).unwrap().as_str().to_string())
        }

        fn from_collected_data(
            content: String,
            sbd: SharedBehaviorData,
        ) -> Result<Spanned<String>, ParseError> {
            Ok(Spanned::new(sbd.to_span(), content))
        }

        parser.parse_object(&RE, collect_data, from_collected_data)
    }
}
