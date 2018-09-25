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

pub mod affiliated_keywords;
pub mod document;
pub mod elements;
pub mod greater_elements;
pub mod objects;
pub mod parsing;

use self::affiliated_keywords::AffiliatedKeywords;
use mopa::Any;
use std::str::pattern::Pattern;

// TODO
#[allow(dead_code)]
static ORG_LINK_TYPES: () = ();

/// All greater elements, elements and objects share some shared behavior.
///
/// This trait adds getters for the needed properties to the elements/objects. The following
/// properties are needed:
///
/// - **span**: Marks where in the document this element is located. Used for error/warning messages
/// - **post blank**: Blank lines and whitespace at the end of the element.
/// - **parent**: The parent element that contains this one.
///
/// The actual data is stored in the convenience struct [`SharedBehaviorData`]. The implementing
/// structs only need to implement `shared_behavior_data()` and this trait will provide the
/// getters for the fields of the `SharedBehaviorData` struct.
///
/// [`SharedBehaviorData`]: struct.SharedBehaviorData.html
pub trait SharedBehavior: Any {
    /// Returns a reference to the data of the shared behavior.
    ///
    /// You should most likely not use this method. It is just a proxy for the other methods on
    /// this trait.
    ///
    /// Wenn implementing this method you should simply return the field that stores this data.
    fn shared_behavior_data(&self) -> &SharedBehaviorData;

    /// Returns the span of the object or element in the file.
    fn span(&self) -> &Span {
        &self.shared_behavior_data().span
    }

    fn post_blank(&self) -> usize {
        self.shared_behavior_data().post_blank
    }
}
mopafy!(SharedBehavior);

/// Helper struct that contains the data for the shared behavior. See [`SharedBehavior`].
///
/// [`SharedBehavior`]: trait.SharedBehavior.html
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SharedBehaviorData {
    span: Span,
    post_blank: usize,
}

/// Represents where in the file the a object or element is.
///
/// It contains a start and an end. `end` is always bigger than or equal to `start`.
///
/// This is useful for warning/error messages and modifying the file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

/// Some greater elements, elements and objects can contain other objects or elements.
///
/// These elements and objects have the following additional properties:
///
/// - **content span**: Marks where in the document the content begins and ends.
/// - **content**: A list of all elements, objects and raw string contained in this element or
///   object.
///
/// The actual data is stored in the convenience struct [`ContentData`]. The implementing structs
/// only need to implement `content_data()` and this trait will provide the getters for the fields
/// of the `ContentData` struct.
///
/// [`ContentData`]: struct.ContentData.html
pub trait HasContent<T: 'static>: SharedBehavior {
    /// Returns a reference to the data needed to contain objects.
    ///
    /// You should most likely not use this method. It is just a proxy for the other methods on
    /// this trait.
    ///
    /// Wenn implementing this method you should simply return the field that stores this data.
    fn content_data(&self) -> &ContentData<T>;

    fn content_span(&self) -> &Span {
        &self.content_data().span
    }

    fn content(&self) -> &[T] {
        &self.content_data().content
    }
}

/// Helper struct that contains the data for the elements and objects that can contain other
/// objects.
///
/// See [`HasContent`].
///
/// [`HasContent`]: trait.HasContent.html
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ContentData<T> {
    span: Span,
    content: Vec<T>,
}

/// Some greater elements and elements can have affiliated keywords.
///
/// Those elements have to following additional properties:
///
/// - **affiliated keywords span**: Marks where in the document the affiliated keywords are
///   location.
/// - **affiliated keywords**: Contains all affiliated keywords for this element.
///
/// The actual data is stored in the convenience struct [`AffiliatedKeywordsData`]. The
/// implementing structs only need to implement `affiliated_keywords_data()` and this trait will
/// provide the getters for the fields of the `AffiliatedKeywordsData` struct.
///
/// [`AffiliatedKeywordsData`]: struct.AffiliatedKeywordsData.html
pub trait HasAffiliatedKeywords: Element {
    /// Returns a reference to the data needed to have affiliated keywords.
    ///
    /// You should most likely not use this method. It is just a proxy for the other methods on
    /// this trait.
    ///
    /// Wenn implementing this method you should simply return the field that stores this data.
    fn affiliated_keywords_data(&self) -> &AffiliatedKeywordsData;

    fn affiliated_keywords(&self) -> &AffiliatedKeywords {
        &self.affiliated_keywords_data().affiliated_keywords
    }

    fn affiliated_keywords_span(&self) -> &Span {
        &self.affiliated_keywords_data().span
    }
}

/// Helper struct that contains the data for the elements that have affiliated keywords.
///
/// See [`HasAffiliatedKeywords`].
///
/// [`HasAffiliatedKeywords`]: trait.HasAffiliatedKeywords.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AffiliatedKeywordsData {
    affiliated_keywords: AffiliatedKeywords,
    span: Span,
}

/// Represents a value and its position in an org file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpannedValue<T> {
    span: Span,
    value: T,
}

impl<T> SpannedValue<T> {
    pub fn new(span: Span, value: T) -> Self {
        SpannedValue { span, value }
    }
    pub fn span(&self) -> &Span {
        &self.span
    }
    pub fn value(&self) -> &T {
        &self.value
    }
}

/// A secondary string is a list of raw strings and objects.
///
/// It is used for attributes of elements that can contain objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecondaryString<T: AsRawString>(Vec<T>);

impl<T: AsRawString> SecondaryString<T> {
    pub fn new() -> Self {
        SecondaryString(Vec::new())
    }
}

impl<T: AsRawString> Default for SecondaryString<T> {
    fn default() -> SecondaryString<T> {
        SecondaryString::new()
    }
}

pub trait AsRawString {
    fn as_raw_string(&self) -> Option<&str>;

    fn is_raw_string(&self) -> bool {
        self.as_raw_string().is_some()
    }
}

impl<T: AsRawString> SecondaryString<T> {
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

impl<T: AsRawString> PartialEq<str> for SecondaryString<T> {
    fn eq(&self, other: &str) -> bool {
        self.0
            .first()
            .and_then(|x| x.as_raw_string())
            .map(|s| s == other)
            .unwrap_or(false)
    }
}

/// Marker trait for objects in an org file.
///
/// Objects are the smallest units and represent the content of the org file.
pub trait Object: SharedBehavior {}

/// Marker trait for the elements in an org file.
///
/// Elements represent the structure of the org file.
///
/// See [`elements`] module for all available elements.
pub trait Element: SharedBehavior {}

/// Marker trait for the greater elements in an org file.
///
/// Greater elements are elements which can contain other (greater) elements. Usually they can't
/// contain themselfes (see the specific element for more details).
///
/// See [`greater_elements`] module for all available greater elements.
pub trait GreaterElement<T: 'static>: Element + HasContent<T> {}

/// The standard set of objects as defined by org mode.
///
/// These objects are used by most other recursive objects. E.g. a bold text can contain an entity.
#[derive(AsRawString, Debug, Clone, PartialEq, Eq, Hash)]
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

/// The standard set of objects without [`objects::LineBreak`]s.
///
/// Used for elements that can contain the standard set but no line breaks. E.g.
/// [`greater_elements::Headline`] or [`greater_elements::Inlinetask`].
#[derive(AsRawString, Debug, Clone, PartialEq, Eq, Hash)]
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

/// This is a list of elements and greater elements.
///
/// This is used for the [`ContentData`] of [`greater_elements`]. Note that greater elements can't
/// usually directly contain elements of the same type. So this is not strictly type safe. E.g. a
/// drawer can't contain a drawer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementSet {
    BabelCall(elements::BabelCall),
    CenterBlock(greater_elements::CenterBlock),
    Clock(elements::Clock),
    Comment(elements::Comment),
    CommentBlock(elements::CommentBlock),
    DiarySexp(elements::DiarySexp),
    Drawer(greater_elements::Drawer),
    DynamicBlock(greater_elements::DynamicBlock),
    ExampleBlock(elements::ExampleBlock),
    ExportBlock(elements::ExportBlock),
    FixedWidth(elements::FixedWidth),
    FootnoteDefinition(greater_elements::FootnoteDefinition),
    HorizontalRule(elements::HorizontalRule),
    Inlinetask(greater_elements::Inlinetask),
    Keyword(elements::Keyword),
    LatexEnvironment(elements::LatexEnvironment),
    Paragraph(elements::Paragraph),
    PlainList(greater_elements::PlainList),
    Planning(elements::Planning),
    PropertyDrawer(greater_elements::PropertyDrawer),
    QuoteBlock(greater_elements::QuoteBlock),
    Section(greater_elements::Section),
    SpecialBlock(greater_elements::Section),
    SrcBlock(elements::SrcBlock),
    Table(greater_elements::Table),
    VerseBlock(greater_elements::VerseBlock),
}
