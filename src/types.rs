//! This module contains all types and traits needed to represent an org file.
//!
//! The *elements* of a org file are separated into three categories:
//!
//! - [`Object`]s represent the content of the file.
//! - [`Element`]s represent the structure of the file.
//! - [`GreaterElement`] is a kind of `Element` that can contain other elements.
//!
//! # Implementation of shared behavior
//!
//! There is some shared behavior between elements an objects. This is organized into the
//! traits: [`SharedBehavior`], [`ContainsObjects`] and [`HasAffiliatedKeywords`]. Those traits
//! rely on specific data being stored in the elements/objects. To simplify this the data is
//! stored in helper traits and these helper traits are then stored in elements/objects. The
//! element/object structs only need to implement a getter method for the helper struct and the
//! trait will give them getter methods for the data in those helper structs.

use std::collections::HashMap;

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
pub trait SharedBehavior {
    /// Returns a reference to the data of the shared behavior.
    ///
    /// You should most likely not use this method. It is just a proxy for the other methods on
    /// this trait.
    ///
    /// Wenn implementing this method you should simply return the field that stores this data.
    fn shared_behavior_data(&self) -> &SharedBehaviorData;

    fn span(&self) -> &Span {
        &self.shared_behavior_data().span
    }

    fn post_blank(&self) -> &u32 {
        &self.shared_behavior_data().post_blank
    }

    fn parent(&self) -> &Option<ParentId> {
        &self.shared_behavior_data().parent
    }
}

/// Helper struct that contains the data for the shared behavior. See [`SharedBehavior`].
pub struct SharedBehaviorData {
    span: Span,
    post_blank: u32,
    parent: Option<ParentId>,
}

/// Represents where in the file the a object or element is.
///
/// It contains a start and an end. `end` is always bigger than or equal to `start`.
///
/// This is useful for warning/error messages and modifying the file.
pub struct Span {
    start: u64,
    end: u64,
}

impl Span {
    pub fn new(start: u64, end: u64) -> Self {
        Span { start, end }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }
}

// TODO
pub struct ParentId;

/// Some greater elements, elements and objects can contain other objects. These elements and
/// objects have the following additional properties:
///
/// - **content span**: Marks where in the document the content begins and ends.
/// - **content**: A list of all elements, objects and raw string contained in this element or
///   object.
///
/// The actual data is stored in the convenience struct [`ContentData`]. The implementing structs
/// only need to implement `content_data()` and this trait will provide the getters for the fields
/// of the `ContentData` struct.
pub trait ContainsObjects: SharedBehavior {
    /// Returns a reference to the data needed to contain objects.
    ///
    /// You should most likely not use this method. It is just a proxy for the other methods on
    /// this trait.
    ///
    /// Wenn implementing this method you should simply return the field that stores this data.
    fn content_data(&self) -> &ContentData;

    fn content_span(&self) -> &Span {
        &self.content_data().span
    }

    fn content(&self) -> &Vec<ObjectId> {
        &self.content_data().content
    }
}

/// Helper struct that contains the data for the elements and objects that can contain other
/// objects.
///
/// See [`ContainsObjects`].
pub struct ContentData {
    span: Span,
    content: Vec<ObjectId>,
}

/// This is an id in the storage engine.
pub struct ObjectId;

/// Some greater elements and elements can have affiliated keywords. Those elements have to
/// following additional properties:
///
/// - **affiliated keywords span**: Marks where in the document the affiliated keywords are
///   location.
/// - **affiliated keywords**: Contains all affiliated keywords for this element.
///
/// The actual data is stored in the convenience struct [`AffiliatedKeywordsData`]. The
/// implementing structs only need to implement `affiliated_keywords_data()` and this trait will
/// provide the getters for the fields of the `AffiliatedKeywordsData` struct.
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
pub struct AffiliatedKeywordsData {
    affiliated_keywords: AffiliatedKeywords,
    span: Span,
}

/// Contains all affiliated keywords for one element/object.
///
/// An affiliated keyword represents an attribute of an element.
///
/// Not all elements can have affiliated keywords. See the specific element.
///
/// Affiliated keywords have one of the following formats:
///
/// - `#+KEY: VALUE`
/// - `#+KEY[OPTIONAL]: VALUE`
/// - `#+ATTR_BACKEND: VALUE`
pub struct AffiliatedKeywords {
    /// Parsed from: `#+CAPTION[OPTIONAL]: VALUE`.
    ///
    /// Where `OPTIONAL` (and the brackets) are optional and both `OPTIONAL` and `VALUE` are
    /// secondary strings (can contain objects).
    ///
    /// The caption key can occur more than once.
    pub caption: Vec<SpannedValue<(Option<SecondaryString>, SecondaryString)>>,
    /// Parsed from: `#+HEADER: VALUE`.
    ///
    /// The header key can occur more than once.
    ///
    /// The deprecated `HEADERS` key will also be parsed to this variant.
    pub header: Vec<SpannedValue<String>>,
    /// Parsed from: `#+NAME: VALUE`.
    ///
    /// The deprecated `LABEL`, `SRCNAME`, `TBLNAME`, `DATA`, `RESNAME` and `SOURCE` keys will also
    /// be parsed to this variant.
    pub name: Option<SpannedValue<String>>,
    /// Parsed from: `#+PLOT: VALUE`.
    pub plot: Option<SpannedValue<String>>,
    /// Parsed from: `#+RESULTS[OPTIONAL]: VALUE`.
    ///
    /// Where `OPTIONAL` (and the brackets) are optional and both `OPTIONAL` and `VALUE` are
    /// secondary strings (can contain objects).
    ///
    /// The deprecated `RESULT` key will also be parsed to this variant.
    pub results: Option<SpannedValue<(Option<String>, String)>>,
    /// Parsed from: `#+ATTR_BACKEND: VALUE`.
    ///
    /// The attr keywords for one backend can occur more than once.
    pub attrs: HashMap<String, Vec<SpannedValue<String>>>,
}

/// Represents a value and its position in an org file.
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

pub struct SecondaryString(Vec<ObjectId>);

/// Marker trait for objects in an org file.
///
/// Objects are the smallest units and represent the content of the org file.
pub trait Object: SharedBehavior {}

/// Marker trait for the elements in an org file.
///
/// Elements represent the structure of the org file.
///
/// Elements are:
///
/// - all elements listen in [`GreaterElement`]
/// - [`BabelCall`]
/// - [`Clock`]
/// - [`Comment`]
/// - [`CommentBlock`]
/// - [`DiarySexp`]
/// - [`ExampleBlock`]
/// - [`ExportBlock`]
/// - [`FixedWidth`]
/// - [`HorizontalRule`]
/// - [`Keyword`]
/// - [`LatexEnvironment`]
/// - [`NodeProperty`]
/// - [`Paragraph`]
/// - [`Planning`]
/// - [`SrcBlock`]
/// - [`TableRow`]
/// - [`VerseBlock`]
pub trait Element: SharedBehavior {}

/// Marker trait for the greater elements in an org file.
///
/// Greater elements are elements which can contain other (greater) elements. Usually they can't
/// contain themselfes.
///
/// Greater elements are:
///
/// - [`CenterBlock`]
/// - [`Drawer`]
/// - [`DynamicBlock`]
/// - [`FootnoteDefinition`]
/// - [`Headline`]
/// - [`Inlinetask`]
/// - [`Item`]
/// - [`PlainList`]
/// - [`PropertyDrawer`]
/// - [`QuoteBlock`]
/// - [`Section`]
/// - [`SpecialBlock`]
/// - [`Table`]
pub trait GreaterElement: Element + ContainsObjects {}

use self::elements::*;

/// Contains all elements.
///
/// TODO maybe separate in elements and greater elements.
pub mod elements {
    use super::*;

    // TODO make a procedural macro to derive all this

    /// A babel call element.
    ///
    /// Used to execute [`SrcBlock`]s and put their results into the org file.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+CALL: FUNCTION[INSIDE-HEADER](ARGUMENTS) END-HEADER
    /// ```
    ///
    /// `FUNCTION` is the name of a [`SrcBlock`] to execute. `INSIDE-HEADER`, `ARGUEMENTS` and
    /// `END-HEADER` can contain everything except a newline (and their respective closing char).
    pub struct BabelCall {
        shared_behavior_data: SharedBehaviorData,
        content_data: ContentData,
        affiliated_keywords_data: AffiliatedKeywordsData,

        /// The code block to call
        call: String,
        inside_header: String,
        arguments: String,
        end_header: String,
        // raw_value: String,
    }

    impl BabelCall {
        pub fn call(&self) -> &str {
            &self.call
        }
        pub fn inside_header(&self) -> &str {
            &self.inside_header
        }
        pub fn arguments(&self) -> &str {
            &self.arguments
        }
        pub fn end_header(&self) -> &str {
            &self.end_header
        }
    }

    impl SharedBehavior for BabelCall {
        fn shared_behavior_data(&self) -> &SharedBehaviorData {
            &self.shared_behavior_data
        }
    }
    impl ContainsObjects for BabelCall {
        fn content_data(&self) -> &ContentData {
            &self.content_data
        }
    }
    impl Element for BabelCall {}
    impl GreaterElement for BabelCall {}
    impl HasAffiliatedKeywords for BabelCall {
        fn affiliated_keywords_data(&self) -> &AffiliatedKeywordsData {
            &self.affiliated_keywords_data
        }
    }

    pub struct CenterBlock;
    pub struct Clock;
    pub struct Comment;
    pub struct CommentBlock;
    pub struct DiarySexp;
    pub struct Drawer;
    pub struct DynamicBlock;
    pub struct ExampleBlock;
    pub struct ExportBlock;
    pub struct FixedWidth;
    pub struct FootnoteDefinition;
    pub struct Headline;
    pub struct HorizontalRule;
    pub struct Inlinetask;
    pub struct Item;
    pub struct Keyword;
    pub struct LatexEnvironment;
    pub struct NodeProperty;
    pub struct Paragraph;
    pub struct PlainList;
    pub struct Planning;
    pub struct PropertyDrawer;
    pub struct QuoteBlock;
    pub struct Section;
    pub struct SpecialBlock;
    pub struct SrcBlock;
    pub struct Table;
    pub struct TableRow;
    pub struct VerseBlock;
}
