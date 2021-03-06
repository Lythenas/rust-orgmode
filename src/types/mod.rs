//! This module contains all types and traits needed to represent an org file.
//!
//! The *elements* of a org file are separated into three categories:
//!
//! - [`Object`]s represent the content of the file.
//! - [`Element`]s represent the structure of the file.
//! - [`GreaterElement`] is a kind of [`Element`] that can contain other elements.
//!
//! # Implementation of shared behavior
//!
//! There is some shared behavior between elements an objects. This is organized into the
//! traits: [`SharedBehavior`], [`HasContent`] and [`HasAffiliatedKeywords`]. Those traits
//! rely on specific data being stored in the elements/objects. To simplify this the data is
//! stored in helper traits and these helper traits are then stored in elements/objects. The
//! element/object structs only need to implement a getter method for the helper struct and the
//! trait will give them getter methods for the data in those helper structs.
//!
//! [`Object`]: `types::Object`
//! [`Element`]: `types::Element`
//! [`GreaterElement`]: `types::GreaterElement`
//! [`SharedBehavior`]: `types::SharedBehavior`
//! [`HasContent`]: `types::HasContent`
//! [`HasAffiliatedKeywords`]: `types::HasAffiliatedKeywords`

pub mod affiliated_keywords;
pub mod document;
pub mod elements;
pub mod greater_elements;
pub mod objects;

use self::affiliated_keywords::AffiliatedKeywords;
use itertools::Itertools;
use std::fmt;
use std::str::pattern::Pattern;

// TODO
#[allow(dead_code)]
static ORG_LINK_TYPES: () = ();

/// Represents where in the file the a object or element is.
///
/// It contains a start and an end. `end` is always bigger than or equal to `start`. Span is to be
/// interpreted as the numbers in a [`RangeFull`]. `start` is part of the span but `end` is not.
///
/// ```text
/// some text
/// ^^^
/// ```
///
/// The marked text (`som`) has the span: **0 to 3** not 0 to 2.
///
/// This is useful for warning/error messages and modifying the file.
///
/// [`RangeFull`]: `std::ops::RangeFull`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'i> From<pest::Span<'i>> for Span {
    fn from(span: pest::Span<'i>) -> Self {
        Span::new(span.start(), span.end())
    }
}

/// Some greater elements, elements and objects can contain other objects or elements.
///
/// These are then called parents to those other elements or objects.
pub trait Parent<T>: crate::private::Sealed {
    /// Returns the spanned content or `None` if there is no content.
    fn content(&self) -> Option<&Spanned<T>> {
        None
    }
}

/// Some greater elements and elements can have affiliated keywords.
pub trait HasAffiliatedKeywords: Element {
    /// Returns the affiliated keywords or `None` if there are none.
    fn affiliated_keywords(&self) -> Option<&Spanned<AffiliatedKeywords>>;
}

/// Represents a value and its [`Span`] (beginning and end position) in an org file.
///
/// # Todo
///
/// Wrap the `Span` in an `Option` to represent the case where this element has been created
/// artificially and is not part of a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    value: T,
    span: Option<Span>,
}

impl<T> Spanned<T> {
    pub fn new(value: T) -> Self {
        Spanned { span: None, value }
    }
    pub fn with_span(value: T, span: Span) -> Self {
        Spanned {
            value,
            span: Some(span),
        }
    }
    pub fn with_optional_span(value: T, span: Option<Span>) -> Self {
        Spanned { value, span }
    }
    pub fn span(&self) -> &Option<Span> {
        &self.span
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn to_value(self) -> T {
        self.value
    }
    pub fn map_value<F, U>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }
    pub fn get_mut_value(&mut self) -> &mut T {
        &mut self.value
    }
}

trait IntoSpanned<T> {
    fn into_spanned(self, span: Option<Span>) -> Spanned<T>;
}

impl<T> IntoSpanned<T> for T {
    fn into_spanned(self, span: Option<Span>) -> Spanned<T> {
        Spanned::with_optional_span(self, span)
    }
}

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// A secondary string is a list of raw strings and objects.
///
/// It is used for attributes of elements that can contain objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecondaryString<T: AsRawString>(Vec<T>);

impl<T: fmt::Display + AsRawString> fmt::Display for SecondaryString<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().format(" "))
    }
}

impl<T: AsRawString> SecondaryString<T> {
    pub fn new() -> Self {
        SecondaryString(Vec::new())
    }
    pub fn with_one(t: T) -> Self {
        SecondaryString(vec![t])
    }

    /// Returns `true` if this `SecondaryString` starts with a raw string and the given pattern matches
    /// a prefix of this string.
    ///
    /// Returns `false` if it does not.
    pub fn starts_with<'a, P>(&'a self, pat: P) -> bool
    where
        P: Pattern<'a>,
    {
        self.0
            .first()
            .and_then(|x| x.as_raw_string())
            .map(|s| s.starts_with(pat))
            .unwrap_or(false)
    }
}

impl<T: AsRawString> Default for SecondaryString<T> {
    fn default() -> SecondaryString<T> {
        SecondaryString::new()
    }
}

impl<T: AsRawString> PartialEq<str> for SecondaryString<T> {
    fn eq(&self, other: &str) -> bool {
        self.0
            .first()
            .and_then(|x| x.as_raw_string())
            .map(|s| s == other)
            .unwrap_or(false)
    }
}

/// A cheap conversion to a raw [`str`] that may fail.
///
/// Used for objects and raw strings in [`SecondaryString`]. Abstracts away the type of the actual
/// objects stored when e.g. for [`starts_with()`][`SecondaryString::starts_with`].
pub trait AsRawString {
    /// Performs the conversion.
    fn as_raw_string(&self) -> Option<&str>;
}

/// Marker trait for objects in an org file.
///
/// Objects are the smallest units and represent the content of the org file.
pub trait Object: crate::private::Sealed {}

/// Marker trait for the elements in an org file.
///
/// Elements represent the structure of the org file.
///
/// See [`elements`] module for all available elements.
pub trait Element: crate::private::Sealed {}

/// Marker trait for the greater elements in an org file.
///
/// Greater elements are (usually) elements which can contain other (greater) elements. Usually they
/// can't contain themselves (see the specific element for more details).
///
/// See [`greater_elements`] module for all available greater elements.
pub trait GreaterElement: Element {}

/// The standard set of objects as defined by org mode.
///
/// These objects are used by most other recursive objects. E.g. a bold text can contain an entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardSet {
    RawString(String),
    Entity(objects::Entity),
    ExportSnippet(objects::ExportSnippet),
    FootnoteReference(objects::FootnoteReference),
    InlineBabelCall(objects::InlineBabelCall),
    InlineSrcBlock(objects::InlineSrcBlock),
    LatexFragment(objects::LatexFragment),
    LineBreak(objects::LineBreak),
    Link(objects::Link),
    Macro(objects::Macro),
    RadioTarget(objects::RadioTarget),
    StatisticsCookie(objects::StatisticsCookie),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    Target(objects::Target),
    TextMarkup(objects::TextMarkup),
    Timestamp(objects::Timestamp),
}

impl fmt::Display for StandardSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StandardSet::RawString(s) => write!(f, "{}", s),
            _ => unimplemented!(),
        }
    }
}

impl AsRawString for StandardSet {
    fn as_raw_string(&self) -> Option<&str> {
        if let StandardSet::RawString(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

/// The standard set of objects without [`LineBreak`]s.
///
/// Used for elements that can contain the standard set but no line breaks. E.g.
/// [`Headline`] or [`Inlinetask`].
///
/// [`LineBreak`]: `objects::LineBreak`
/// [`Headline`]: `greater_elements::Headline`
/// [`Inlinetask`]: `greater_elements::Inlinetask`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardSetNoLineBreak {
    RawString(String),
    Entity(objects::Entity),
    ExportSnippet(objects::ExportSnippet),
    FootnoteReference(objects::FootnoteReference),
    InlineBabelCall(objects::InlineBabelCall),
    InlineSrcBlock(objects::InlineSrcBlock),
    LatexFragment(objects::LatexFragment),
    Link(objects::Link),
    Macro(objects::Macro),
    RadioTarget(objects::RadioTarget),
    StatisticsCookie(objects::StatisticsCookie),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    Target(objects::Target),
    TextMarkup(objects::TextMarkup),
    Timestamp(objects::Timestamp),
}

impl AsRawString for StandardSetNoLineBreak {
    fn as_raw_string(&self) -> Option<&str> {
        if let StandardSetNoLineBreak::RawString(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

/// This is a list of elements and greater elements.
///
/// This is used for the content of [`greater_elements`]. Note that greater elements can't
/// usually directly contain elements of the same type. So this is not strictly type safe. E.g. a
/// drawer can't contain a drawer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementSet {
    BabelCall(Box<elements::BabelCall>),
    CenterBlock(Box<greater_elements::CenterBlock>),
    Clock(Box<elements::Clock>),
    Comment(Box<elements::Comment>),
    CommentBlock(Box<elements::CommentBlock>),
    DiarySexp(Box<elements::DiarySexp>),
    Drawer(Box<greater_elements::Drawer>),
    DynamicBlock(Box<greater_elements::DynamicBlock>),
    ExampleBlock(Box<elements::ExampleBlock>),
    ExportBlock(Box<elements::ExportBlock>),
    FixedWidth(Box<elements::FixedWidth>),
    FootnoteDefinition(Box<greater_elements::FootnoteDefinition>),
    HorizontalRule(Box<elements::HorizontalRule>),
    Inlinetask(Box<greater_elements::Inlinetask>),
    Keyword(Box<elements::Keyword>),
    LatexEnvironment(Box<elements::LatexEnvironment>),
    Paragraph(Box<elements::Paragraph>),
    PlainList(Box<greater_elements::PlainList>),
    Planning(Box<elements::Planning>),
    PropertyDrawer(Box<greater_elements::PropertyDrawer>),
    QuoteBlock(Box<greater_elements::QuoteBlock>),
    //Section(Box<greater_elements::Section>),
    SpecialBlock(Box<greater_elements::SpecialBlock>),
    SrcBlock(Box<elements::SrcBlock>),
    Table(Box<greater_elements::Table>),
    VerseBlock(Box<greater_elements::VerseBlock>),
}

impl From<elements::Paragraph> for ElementSet {
    fn from(paragraph: elements::Paragraph) -> Self {
        ElementSet::Paragraph(Box::new(paragraph))
    }
}
