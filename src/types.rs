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
//! traits: [`SharedBehavior`], [`ContainsObjects`] and [`HasAffiliatedKeywords`]. Those traits
//! rely on specific data being stored in the elements/objects. To simplify this the data is
//! stored in helper traits and these helper traits are then stored in elements/objects. The
//! element/object structs only need to implement a getter method for the helper struct and the
//! trait will give them getter methods for the data in those helper structs.
//!
//! [`Object`]: trait.Object.html
//! [`Element`]: trait.Element.html
//! [`GreaterElement`]: trait.GreaterElement.html
//! [`SharedBehavior`]: trait.SharedBehavior.html
//! [`ContainsObjects`]: trait.ContainsObjects.html
//! [`HasAffiliatedKeywords`]: trait.HasAffiliatedKeywords.html

use std::collections::HashMap;

// TODO using lazy_static
#[allow(dead_code)]
static ORG_ENTITIES: () = ();
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
///
/// [`SharedBehavior`]: trait.SharedBehavior.html
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

/// This represents a parent in the storage engine (TODO).
pub struct ParentId;

/// Some greater elements, elements and objects can contain other objects.
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
///
/// [`ContainsObjects`]: trait.ContainsObjects.html
pub struct ContentData {
    span: Span,
    content: Vec<ObjectId>,
}

/// This is an id in the storage engine (TODO).
pub struct ObjectId;

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

/// A secondary string is a list of raw strings and objects.
///
/// It is used for attributes of elements that can contain objects.
pub struct SecondaryString(Vec<ObjectId>);

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
pub trait GreaterElement: Element + ContainsObjects {}

/// Contains all elements except [`greater_elements`].
pub mod elements {
    use super::*;
    use rust_orgmode_derive::add_fields_for;

    /// A babel call element.
    ///
    /// # Sematics
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
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct BabelCall {
        /// The code block to call
        call: String,
        inside_header: String,
        arguments: String,
        end_header: String,
    }

    /// A clock element.
    ///
    /// # Sematics
    ///
    /// TODO
    ///
    /// # Syntax
    ///
    /// ```text
    /// CLOCK: TIMESTAMP DURATION
    /// ```
    ///
    /// `TIMESTAMP` and `DURATION` are optional. `TIMESTAMP` is a [`objects::Timestamp`]. `DURATION`
    /// follows the pattern `=> HH:MM` where `HH` is a number containing any number of digits and
    /// `MM` is a two digit number.
    #[derive(Element, getters)]
    #[add_fields_for(SharedBehavior)]
    pub struct Clock {
        timestamp: Option<objects::Timestamp>,
        duration: Option<(u64, u8)>,
    }

    impl Clock {
        pub fn status(&self) -> ClockStatus {
            match self.duration {
                Some(_) => ClockStatus::Closed,
                None => ClockStatus::Running,
            }
        }
    }

    pub enum ClockStatus {
        Running,
        Closed,
    }

    /// A comment element.
    ///
    /// # Semantics
    ///
    /// Comments are ignored when parsing. They are not actually ignored, they just don't have any
    /// meaning.
    ///
    /// # Snytax
    ///
    /// A line starting with `#` and space (or end of line). The `#` can be optionally preceded
    /// with whitespace.
    ///
    ///
    /// ```text
    /// # CONTENTS
    /// ```
    ///
    /// `CONTENTS` can be any string.
    ///
    /// Consecutive comment lines are accumulated into one comment.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct Comment {
        value: String,
    }

    /// A comment block.
    ///
    /// # Semantics
    ///
    /// See [`Comment`].
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_COMMENT
    /// CONTENTS
    /// #+END_COMMENT
    /// ```
    ///
    /// `CONTENTS` can contain anything except a line `#+END_COMMENT` on its own. Lines beginning
    /// with stars must be quoted by a comma. `CONTENTS` will not be parsed.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct CommentBlock {
        value: String,
    }

    /// A diary sexp.
    ///
    /// # Semantics
    ///
    /// TODO
    ///
    /// # Syntax
    ///
    /// ```text
    /// %%(VALUE
    /// ```
    ///
    /// `VALUE` can contain any character except a newline. The expression has to start at the
    /// beginning of the line.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct DiarySexp {
        value: String,
    }

    /// An example block.
    ///
    /// # Semantics
    ///
    /// Its content will not be parsed. Examples are typeset in monospace when exporting.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_EXAMPLE FLAGS
    /// CONTENTS
    /// #+END_EXAMPLE
    /// ```
    ///
    /// `CONTENTS` can contain anything except a line `#+END_EXAMPLE` on its own. Lines beginning
    /// with stars must be quoted by comma. `CONTENTS` will not be parsed. `CONTENT` can also
    /// contain labels with the pattern `(ref:LABEL)`. **Labels are not recognized.**
    ///
    /// `FLAGS` see [`BlockFlags`].
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct ExampleBlock {
        value: String,
        flags: BlockFlags,
    }

    /// Contains the flags of an [`ExampleBlock`] or [`SrcBlock`].
    ///
    /// Can contain the following flags:
    ///
    /// - `+n AMOUNT`: continued number lines, will continue the numbering of the previos numbered
    ///   snippet. `AMOUNT` will be added to the last line of the previod block to determine the
    ///   number of the first line.
    /// - `-n AMOUNT`: new number lines (`AMOUNT` is the start line number of the block)
    /// - `-i`: preserve indent
    /// - `-r`: removes the labels when exporting. References will use line numbers.
    /// - `-k`: don't use labels
    /// - `-l "FMT"`: label format (if the default format conflicts with the language you are
    ///   using)
    ///
    /// `AMOUNT` is an optional positive number.
    ///
    /// `FMT` can contain everything except `"` and newlines.
    #[derive(getters)]
    pub struct BlockFlags {
        number_lines: Option<NumberLinesFlag>,
        /// Default: false
        preserve_indent: bool,
        /// Default: true
        ///
        /// If true, code-references should use labels instead of line numbers.
        retain_labels: bool,
        label_fmt: Option<String>,
    }

    pub enum NumberLinesFlag {
        Continued(Option<u64>),
        New(Option<u64>),
    }

    /// An export block.
    ///
    /// # Semantics
    ///
    /// TODO
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_EXPORT BACKEND
    /// CONTENTS
    /// #+END_EXPORT
    /// ```
    ///
    /// `CONTENTS` can contain anything except a line `#+END_EXAMPLE` on its own. Lines beginning
    /// with stars must be quoted by comma. `CONTENTS` will not be parsed.
    ///
    /// `BACKEND` can contain any alpha-numerical character. Case is ignored.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct ExportBlock {
        value: String,
        /// Always lowercase.
        backend: String,
    }

    /// A fixed width area.
    ///
    /// # Semantics
    ///
    /// Can be used in lists or text for examples. Similar to [`ExampleBlock`] but can be indented.
    ///
    /// # Syntax
    ///
    /// A line beginning with `:` followed by a whitespace or end of line. The `:` can be preceded
    /// by whitespace.
    ///
    /// Consecutive fixed width lines are accumulated.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct FixedWidth {
        value: String,
    }

    /// A horizontal line.
    ///
    /// # Semantics
    ///
    /// A horizontal line.
    ///
    /// # Syntax
    ///
    /// A line of at least 5 consecutive hyphens. Can be precesed by whitespace.
    ///
    /// ```text
    /// -----
    /// ```
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct HorizontalRule {
    }

    /// A keyword.
    ///
    /// # Semantics
    ///
    /// A keywords is similar to [`AffiliatedKeywords`] but they don't belong to another element.
    /// Orphaned affiliated keywords are considered regular keywords.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+KEY: VALUE
    /// ```
    ///
    /// `KEY` can contain any non-whitespace character. But it can't be equal to `CALL` or any
    /// affiliated keyword.
    ///
    /// `VALUE` can contain any character except a newline.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct Keyword {
        key: String,
        value: String,
    }

    /// A document property keyword.
    ///
    /// # Semantics
    ///
    /// See [`Keyword`] but for the whole org file.
    ///
    /// # Syntax
    ///
    /// See [`Keyword`].
    ///
    /// `VALUE` is parsed as a [`SecondaryString`].
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct DocumentProperty {
        key: String,
        value: SecondaryString,
    }

    /// A LaTeX environment.
    ///
    /// # Semantics
    ///
    /// This will be treated as accordingly when exporting with LaTeX. Otherwise it will be treated
    /// as plain text.
    ///
    /// # Syntax
    ///
    /// ```text
    /// \begin{ENVIRONMENT}
    /// CONTENTS
    /// \end{ENVIRONMENT}
    /// ```
    ///
    /// `ENVIRONMENT` can contain any alpha-numeric character and asterisks. Usually the asterisk
    /// is only at the end.
    ///
    /// `CONTENT` can be anything except `\end{ENVIRONMENT}`.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct LatexEnvironment {
        /// Contains everything including `\begin...` and `\end`.
        value: String,
    }

    /// A node property.
    ///
    /// # Semantics
    ///
    /// A property contained in a [`greater_elements::PropertyDrawer`].
    ///
    /// # Syntax
    ///
    /// Follows one of these patterns:
    ///
    /// - `:NAME: VALUE`
    /// - `:NAME+: VALUE`
    /// - `:NAME:`
    /// - `:NAME+:`
    ///
    /// `NAME` can contain any non-whitespace character but can't be an empty string or end with a
    /// plus sign (`+`).
    ///
    /// `VALUE` can contain anything but a newline character.
    #[derive(Element, getters)]
    #[add_fields_for(SharedBehavior)]
    pub struct NodeProperty {
        name: String,
        value: String,
    }

    /// A paragraph.
    ///
    /// # Semantics
    ///
    /// A paragraph is a list of strings and objects ([`SecondaryString`]). Line breaks in the text
    /// are ignored and only [`objects::LineBreak`] will be recognized as a line break.
    ///
    /// # Syntax
    ///
    /// Everything that is not another element is paragraph. Empty lines and other elements end
    /// paragraphs but all inner elements of the current paragraph must be closed first.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct Paragraph {
        // TODO figure out how to actually handle secondary strings
    }

    /// A planning element.
    ///
    /// # Semantics
    ///
    /// Contains the deadline, scheduled and closed timestamps for a headline. All are optional.
    ///
    /// # Syntax
    ///
    /// ```text
    /// KEYWORD: TIMESTAMP
    /// ```
    ///
    /// `KEYWORD` is one of `DEADLINE`, `SCHEDULED` or `CLOSED`. Planning can be repeated but one
    /// keywords can only be used once. The order doesn't matter.
    ///
    /// `TIMESTAMP` is a [`objects::Timestamp`].
    ///
    /// Consecutive planning items are aggregated into one.
    #[derive(Element, getters)]
    #[add_fields_for(SharedBehavior)]
    pub struct Planning {
        closed: Option<objects::Timestamp>,
        deadline: Option<objects::Timestamp>,
        scheduled: Option<objects::Timestamp>,
    }

    /// A block of source code.
    ///
    /// # Semantics
    ///
    /// Same as [`ExampleBlock`] but usually contains source code. The content will be highlighted
    /// according to the language specified.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_SRC LANGUAGE FLAGS ARGUMENTS
    /// CONTENTS
    /// #+END_SRC
    /// ```
    ///
    /// `CONTENTS` can contain anything except a line `#+END_SRC` on its own. Lines beginning
    /// with stars must be quoted by comma. `CONTENTS` will not be parsed.
    ///
    /// `LANGUAGE` can contain anything except whitespace.
    ///
    /// `FLAGS` see [`BlockFlags`].
    ///
    /// `ARGUMENTS` can contain any character except a newline.
    #[derive(Element, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct SrcBlock {
        language: String,
        flags: BlockFlags,
        arguments: String,
    }

}

/// Contains all greater elements.
pub mod greater_elements {
    use super::*;
    use rust_orgmode_derive::add_fields_for;

    /// A center block.
    ///
    /// # Semantics
    ///
    /// Centers text. Also the content can contain markup.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_CENTER
    /// CONTENTS
    /// #+END_CENTER
    /// ```
    ///
    /// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
    /// with stars must be quoted by comma. `CONTENTS` will not be parsed.
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(Element, HasAffiliatedKeywords)]
    pub struct CenterBlock {
        content_data: ContentData, // TODO only allow the standard set of elements
    }

    /// A drawer to hide content.
    ///
    /// # Semantics
    ///
    /// Used to hide content in the editor and when exporting. Drawers can usually be opened and
    /// closed in the editor.
    ///
    /// # Syntax
    ///
    /// ```text
    /// :NAME:
    /// CONTENTS
    /// :END:
    /// ```
    ///
    /// `NAME` can contain any word-constituent characters, hyphens and underscores.
    ///
    /// `CONTENTS` can contain any element except a [`Headline`] and another drawer.
    ///
    /// Drawers can be indented.
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct Drawer {
        name: String,
        // hiddenp: bool,
    }

    /// A dynamic block.
    ///
    /// # Semantics
    ///
    /// The content of dynamic blocks can be updated automatically by calling the a function with
    /// the given name and parameters. It that function needs the previous content of the block an
    /// extra parameter `:content` has to be added.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN: NAME PARAMETERS
    /// CONTENTS
    /// #+END:
    /// ```
    ///
    /// Note the `:` after `BEGIN` and `END`. It can be omitted after `END` without generating an
    /// error.
    ///
    /// `NAME` can contain anything except whitespace.
    ///
    /// `PARAMETERS` can contain any character and can be omitted. They are usually of the format
    /// `:name value` or `:name`.
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct DynamicBlock {
        /// The name of the function that can update this block.
        name: String,
        /// The parameters to pass to the function updating this block.
        ///
        /// Usually of the format `:name value`, separated by a space. Value can also be omitted.
        ///
        /// If the function needs the current content of the block add a parameter `:content`.
        parameters: String, // TODO maybe parse this as a list
        // hiddenp: bool
    }

    /// A footnote definition.
    ///
    /// # Semantics
    ///
    /// Defines a footnote that can be references with a [`objects::FootnoteReference`].
    ///
    /// # Syntax
    ///
    /// ```text
    /// [LABEL] CONTENTS
    /// ```
    ///
    /// `LABEL` is either a number or follows the pattern `fn:WORD` where `WORD` can contain any
    /// word-constituent character, hyphens and underscores.
    ///
    /// `CONTENTS` can contain any element except another footnote definition and a [`Headline`].
    /// It ends at the next footnote definition, headline, with two consecutive empty lines or the
    /// end of the buffer.
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct FootnoteDefinition {
        label: String,
        // pre_blank: u32 // blank lines after `[LABEL]`
    }

    /// A headline.
    ///
    /// # Semantics
    ///
    /// The main element used to structure an org file. Also used as todo items/tasks. Can be
    /// assigned a [`elements::Planning`] item to schedule an event.
    ///
    /// If the first word of `TITLE` is `COMMENT` the headline will be considered as commented
    /// (case is significant). If `TITLE` is `org-footnote-section` it will be considered as
    /// the footnote section (case is significant).
    ///
    /// If `TAGS` contains the `ARCHIVE` tag the headline will be considered archived (case is
    /// significant).
    ///
    /// # Syntax
    ///
    /// ```text
    /// STARS KEYWORD PRIORITY TITLE TAGS
    /// ```
    ///
    /// `STARS` is a string consisting of asterisks only. It has to start at the beginning of the
    /// line. It contains at least one asterisk. This is the only required part of a headline. If
    /// other parts of the headline follow there has to be at least a space after the stars.
    ///
    /// `KEYWORD` is a todo keyword in all capital letters. If other parts of the headline follow
    /// there has to be a single space after the keyword.
    ///
    /// `PRIORITY` is a priority cookie of the form `[#A]` where `A` can be any letter. Capital
    /// letters are recommended.
    ///
    /// `TITLE` can contain any character but a newline. Title will be parsed as secondary string
    /// and can contain the standard set of objects without line breaks.
    ///
    /// `TAGS` is made of strings containing any alpha-numeric character, underscores, at signs,
    /// hash signs and percent signs. Tags are separated and surrounded by `:`s. There can be an
    /// arbitraty amount of whitespace (except newlines) between `TITLE` and `TAGS`. Tags are
    /// usually right aligned at a specified column by the editor.
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct Headline {
        level: u32,
        todo_keyword: Option<TodoKeyword>,
        priority: Option<char>, // TODO maybe make separate struct
        title: Option<SecondaryString>,
        tags: Vec<String>,
        // TODO add reference to the associated planning element
        archived: bool,
        commented: bool,
        footnote_section: bool,
        // TODO add reference to the property drawer

        // quotedp ?
        // hiddenp: bool,
        // pre_blank: u32 // blank lines before the content starts
    }

    /// A todo keyword of a [`Headline`] or [`Inlinetask`].
    ///
    /// Todo keywords can be configured before parsing. The default is to parse `TODO` and `NEXT` as
    /// **`Todo`** and `DONE` as **`Done`**. The actual keyword used is the string in the variant.
    pub enum TodoKeyword {
        /// Usually parsed from `TODO` and `NEXT`.
        Todo(String),
        /// Usually parsed from `DONE`.
        Done(String),
    }

    /// An inline task.
    ///
    /// # Semantics
    ///
    /// Similar to a [`Headline`] but can have a defined end. Headlines end when the next starts or
    /// the document ends.
    ///
    /// # Syntax
    ///
    /// Same syntax as [`Headline`] but starts with at least *org-inlinetask-min-level* astersisks.
    /// This variable is currently not implemented. Inline tasks can also not be commented or
    /// archived.
    ///
    /// Inline tasks can be ended with a line of *org-inlinetask-min-level* asterisks followed by a
    /// space and the string `END`. This should start at the beginning of a line but that is not
    /// required.
    #[derive(GreaterElement, getters)]
    #[add_fields_for(GreaterElement)]
    pub struct Inlinetask {
        todo_keyword: Option<TodoKeyword>,
        priority: Option<char>, // TODO maybe make separate struct
        title: Option<SecondaryString>,
        tags: Vec<String>,

        // hiddenp: bool,
        // pre_blank: u32 // blank lines before the content starts
    }

    /// An item in a list.
    ///
    /// # Semantics
    ///
    /// This is an item in a list.
    ///
    /// # Syntax
    ///
    /// ```text
    /// BULLET [@COUNTER] [CHECKBOX] TAG
    /// ```
    ///
    /// `BULLET` is either an asterisk, a hyphen, a plus sign (for unordered lists) or follows the
    /// pattern `COUNTER.` or `COUNTER)` (for ordered lists). This is the only required part of
    /// item. `BULLET` is always followed by a whitespace or end of line.
    ///
    /// `COUNTER` is a number or a single letter.
    ///
    /// `CHECKBOX` is either a single whitespace, a `X` or a hyphen.
    ///
    /// `TAG` follows the pattern `TAG-TEXT ::` where `TAG-TEXT` can contain any character except a
    /// newline.
    ///
    /// An item ends before the next item, the first line that is less or equally indented that its
    /// starting line or two consecutive empty lines. Indentation of lines within other greater
    /// elements including inlinetask boundaries are ignored.
    #[derive(GreaterElement, getters)]
    #[add_fields_for(GreaterElement)]
    pub struct Item {
        // TODO move all of this to an enum to make more typesafe
        bullet: String, // TODO make struct
        checkbox: Option<Checkbox>,
        counter: Option<String>, // TODO make struct
        tag: Option<String>,
        // structure ?
        // hiddenp: bool
    }

    // TODO find better names for the variants
    /// Checkbox of an [`Item`] in a list.
    pub enum Checkbox {
        /// A space. (Empty checkbox)
        Off,
        /// A `X`. (Checked checkbox)
        X,
        /// A `-`. (Half checked checkbox? or disabled checkbox?)
        Trans,
    }

    /// A plain list.
    ///
    /// # Semantics
    ///
    /// A complete list of [`Item`]s.
    ///
    /// # Syntax
    ///
    /// This is a set of consecutive items of the same indentation. It can only directly contain
    /// items.
    ///
    /// If the dirst item has a `COUNTER` in its `BULLET` the plain list is be an *ordered plain
    /// list*. If it contains a tag it is be a *descriptive list*. Otherwise it is be an
    /// *unordered list*.
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct PlainList {
        // TODO content is only items
        // structure ?
    }

    impl PlainList {
        pub fn kind(&self) -> ListKind {
            // find first item if any
            // get kind of item and return it
            unimplemented!()
        }
    }

    /// The list kind of a [`PlainList`].
    pub enum ListKind {
        Unordered,
        UnorderedDescriptive,
        Ordered,
        OrderedDescriptive,
    }

    /// A property drawer.
    ///
    /// # Semantics
    ///
    /// A drawer associated with a [`Headline`]. It contains attributes of a headline.
    ///
    /// # Syntax
    ///
    /// ```text
    /// :PROPERTIES:
    /// CONTENTS
    /// :END:
    /// ```
    ///
    /// `CONTENTS` consists of zero or more [`elements::NodeProperty`].
    #[derive(GreaterElement, getters)]
    #[add_fields_for(GreaterElement)]
    pub struct PropertyDrawer {
        // TODO make this so only node properties are allowed in the content
        // hiddenp: bool
    }

    /// A quote.
    ///
    /// # Semantics
    ///
    /// Used for quotes. When exporting this block will be indented on the left and right margin.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_QUOTE
    /// CONTENTS
    /// #+END_QUOTE
    /// ```
    ///
    /// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
    /// with stars must be quoted by comma. `CONTENTS` will not be parsed.
    ///
    /// TODO not sure if this is actually a greater element
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct QuoteBlock {
        // hiddenp: bool
    }

    /// A section.
    ///
    /// # Semantics
    ///
    /// This is a container for the content after a [`Headline`] or at the beginning of an org file
    /// before the first headline.
    ///
    /// # Syntax
    ///
    /// A section contains directly any (greater) element. Only a [`Headline`] can contain a
    /// section. Also content before the first headline in a document belongs to a section.
    ///
    /// A section ends at the beginning of the next headline or the end of the file.
    #[derive(GreaterElement, getters)]
    #[add_fields_for(GreaterElement)]
    pub struct Section {
    }

    /// A special block.
    ///
    /// # Semantics
    ///
    /// Any block with name that is not recognized as another block is a special block.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_NAME
    /// CONTENTS
    /// #+END_NAME
    /// ```
    ///
    /// `NAME` can contain any non-whitespace character.
    ///
    /// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
    /// with stars must be quoted by comma. `CONTENTS` will not be parsed.
    ///
    /// TODO not sure if this is actually a greater element
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct SpecialBlock {
        kind: String,
        // hiddenp: bool
    }

    /// A table.
    ///
    /// # Semantics
    ///
    /// There are two types of tables:
    ///
    /// - **org tables** can only contain [`TableRow`]s.
    /// - **table.el tables** don't have parsed content.
    ///
    /// # Syntax
    ///
    /// Tables start with a line starting with a vertical bar or the string `+-` followed by plus
    /// or binus signs only. Tables can be indented. The second line determines what type of table
    /// this is.
    ///
    /// # Org tables
    ///
    /// Org tables start with a line starting with `|` and end at the first line not starting
    /// with a vertical bar. They can be immediately followed by `#+TBLFM: FORMULAS` lines where
    /// `FORMULAS` can contain any character.
    ///
    /// ## Example
    ///
    /// ```text
    /// | col1 | col2 | col3 |
    /// |------+------+------|
    /// |  200 |  300 |  500 |
    /// #+TBLFM: $3=$1+$2
    /// ```
    ///
    /// # Table.el tables
    /// Table.el tables lines start with either a `|` or `+`. And end at the first line not
    /// starting with either a vertical bar or a plus sign.
    ///
    /// ## Example
    ///
    /// ```text
    /// +------+------+------+
    /// | col1 | col2 | col3 |
    /// +------+------+------+
    /// |  200 |  300 |  500 |
    /// +------+------+------+
    /// ```
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
    pub struct Table {
        // TODO maybe make this a bit more type safe
        kind: TableKind,
        /// Empty if this is a org table.
        formulas: Vec<String>,
        /// `None` if this is a org table.
        value: Option<String>,
    }

    /// The kind of a [`Table`].
    pub enum TableKind {
        Org,
        TableEl,
    }

    /// A row in a [`Table`][`Table`].
    ///
    /// # Semantics
    ///
    /// A row contains cell which can contain content.
    ///
    /// # Syntax
    ///
    /// There are two kinds of table rows:
    ///
    /// - normal: vertical bar and any number of [`TableCell`][`objects::TableCell`]s
    ///   ```text
    ///   | cell 1 | cell 2 | ... |
    ///   ```
    /// - a rule: vertical bar followed by hyphens followed by a vertical bar
    ///   ```text
    ///   |--------|
    ///   ```
    #[derive(GreaterElement, getters)]
    #[add_fields_for(SharedBehavior)]
    pub struct TableRow {
        kind: TableRowKind,
        content_data: ContentData, // TODO only allow TableCells
    }

    /// The kind of a [`TableRow`].
    pub enum TableRowKind {
        Normal,
        Rule,
    }

    /// A verse block.
    ///
    /// # Semantics
    ///
    /// Simmilar to an [`elements::ExampleBlock`] but content is interpreted as objects. Verse blocks
    /// preserve indentation.
    ///
    /// # Syntax
    ///
    /// ```text
    /// #+BEGIN_VERSE
    /// CONTENTS
    /// #+END_VERSE
    /// ```
    ///
    /// `CONTENTS` can contain anything except a line `#+END_VERSE` on its own. Lines beginning
    /// with stars must be quoted by comma. `CONTENTS` will be parsed as objects.
    #[derive(GreaterElement, HasAffiliatedKeywords, getters)]
    #[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
    pub struct VerseBlock {
        content_data: ContentData, // TODO only allow the standard set of objects
    }
}

/// Contains all objects.
pub mod objects {
    use super::*;
    use rust_orgmode_derive::add_fields_for;
    use chrono::{NaiveDate, NaiveTime};

    /// An entity.
    ///
    /// # Semantics
    ///
    /// An entity is a special character which has to be exported differently to different formats.
    ///
    /// # Syntax
    ///
    /// ```text
    /// \NAME POST
    /// ```
    ///
    /// `NAME` has to have a valid association in [`ORG_ENTITIES`] or in the used defined variable
    /// `org_entities_user` which can be configured before parsing. It has to conform to the
    /// following regular expression: `(_ +)|(there4|frac[13][24]|[a-zA-Z]+)` (this restriction
    /// could be removed in the future).
    ///
    /// `POST` is the end of line, the string `{}` or a non-alphabetical character (e.g. a
    /// whitespace). It isn't separated from `NAME` by any whitespace.
    ///
    /// TODO implement the org-entities list. See [https://orgmode.org/worg/org-symbols.org]. This
    /// list contains the name, the latex export, the html export, the ascii export, the latin1
    /// export and the utf-8 export.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Entity {
        name: String,
        /// True if the entity ended with `{}`.
        used_brackets: bool,
    }

    /// An export snippet.
    ///
    /// # Semantics
    ///
    /// These snippets are only exported in the specified format. E.g. there can be an export
    /// snippet that is only exported in html.
    ///
    /// # Syntax
    ///
    /// ```text
    /// @@BACKEND:VALUE@@
    /// ```
    ///
    /// `BACKEND` can contain any alpha-numeric character and hyphens.
    ///
    /// `VALUE` can contain anything but the `@@` string.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct ExportSnippet {
        backend: String,
        value: String,
    }

    /// A footnote reference.
    ///
    /// # Semantics
    ///
    /// This is a reference (or link) to a [`greater_elements::FootnoteDefinition`].
    ///
    /// # Syntax
    ///
    /// Follows one of these patterns:
    ///
    /// - normal footnote: `[fn:LABEL]`
    /// - inline footnote: `[fn:LABEL:DEFINITION]`
    ///   can be references by other footnote
    /// - anonymous footnote: `[fn::DEFINITION]`
    ///
    /// `LABEL` can contain any word-constituent character, hyphens and underscores.
    ///
    /// `DEFINITION` can contain any character. Opening and closing square brackets must be
    /// balanced in it. It can contain the standard set of objects, even other footnote references.
    /// Will be parsed as a secondary string and can contain the standard set of objects.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct FootnoteReference {
        // TODO extract enum to make this more type safe
        label: String,
        kind: FootnoteReferenceKind,
        definition: Option<SecondaryString>,
    }

    /// The kind of a [`FootnoteReference`].
    pub enum FootnoteReferenceKind {
        Normal,
        Inline,
        Anonymous,
    }

    /// An inline babe call.
    ///
    /// # Semantics
    ///
    /// Same as [`elements::BabelCall`] but inline.
    ///
    /// # Syntax
    ///
    /// ```text
    /// call_NAME[HEADER](ARGUEMTNS)[HEADER]
    /// ```
    ///
    /// `NAME` can contain any character besides `(`, `[`, whitespace and newline.
    ///
    /// `HEADER` can contain any characer besides `]` and newline.
    ///
    /// `ARGUMENTS` can contain any character besides `)` and newline.
    ///
    /// Both `HEADER`s are optional. But then there are also no square brackets.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct InlineBabelCall {
        call: String,
        inside_header: String,
        arguments: String,
        end_header: String,
    }

    /// An inline src block.
    ///
    /// # Semantics
    ///
    /// Same as [`elements::SrcBlock`] but inline.
    ///
    /// # Syntax
    ///
    /// ```text
    /// src_LANG[OPTIONS]{BODY}
    /// ```
    ///
    /// `LANG` can contain any non-whitespace character.
    ///
    /// `OPTIONS` and `BODY` can contain any character but a newline.
    ///
    /// `OPTIONS` is optional. But then there are also not quote brackets.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct InlineSrcBlock {
        lang: String,
        value: String,
        options: String,
    }

    /// A LaTeX fragment.
    ///
    /// # Semantics
    ///
    /// # Syntax
    ///
    /// Follows one of these patterns:
    ///
    /// ```text
    /// \NAME BRACKETS
    /// \(CONTENTS\)
    /// \[CONTENTS\]
    /// $$CONTENTS$$
    /// PRE$CHAR$POST
    /// PRE$BORDER1 BODY BORDER2$POST
    /// ```
    ///
    /// `NAME` can contain any alphabetical character and can end with an asterisk. `NAME` must not
    /// be on [`ORG_ENTITIES`] or the user defined `org_entities_user` variable otherwise it will
    /// be parsed as a [`Entity`].
    ///
    /// `BRACKETS` is optional and is not separated from `NAME` with whitespace. It can contain any
    /// number of the following patterns (not separated by anything): `[CONTENTS1]`, `{CONTENTS2}`.
    ///
    /// `CONTENTS1` and `CONTENTS2` can contain any character except `{`, `}` and newline.
    /// Additionally `CONTENTS1` can't contain `[` and `]`.
    ///
    /// `CONTENTS` can contain any character but the closing characters of the pattern used.
    ///
    /// `PRE` is either the beginning of the line or any character except `$`.
    ///
    /// `CHAR` is a non-whitspace character except `.`, `,`, `?`, `;`, `'` or `"`.
    ///
    /// `POST` is any punctuation (including parantheses and quotes) or space character or the end
    /// of the line.
    ///
    /// `BORDER1` is any non-whitespace character except `.`, `,`, `;` and `$`.
    ///
    /// `BODY` can contain any character except `$` and may not span over more than 3 lines.
    ///
    /// `BORDER2` is any non-whitespace character except `.`, `,` and `$`.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct LatexFragment {
        /// Contains the entire parsed string, except the `PRE` and `POST` parts.
        value: String,
    }

    /// A line break.
    ///
    /// # Semantics
    ///
    /// Used to export a line break.
    ///
    /// # Syntax
    ///
    /// ```text
    /// \\SPACE
    /// ```
    ///
    /// `SPACE` is zero or more whitespace characters followed by the end of line or end of
    /// document.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct LineBreak {
    }

    /// A link.
    ///
    /// # Semantics
    ///
    /// This is either a link to an internal element or an external website or file.
    ///
    /// # Syntax
    ///
    /// There a 4 formats of links:
    ///
    /// - radio link: `PRE1 RADIO POST1`
    /// - angle link: `<PROTOCOL:PATH>`
    /// - plain link: `PRE2 PROTOCOL:PATH2 POST2`
    /// - bracket link: `[[PATH3]DESCRIPTION]`
    ///
    /// `PRE1` and `POST1` are optional non-alpha-numeric characters.
    ///
    /// `RADIO` is a string matched by a [`RadioTarget`].
    ///
    /// `PROTOCOL` is a string in [`ORG_LINK_TYPES`].
    ///
    /// `PATH` can contain any character except `]`, `<`, `>` and newline.
    ///
    /// `PRE2` and `POST2` are optional non-word-constituent characters.
    ///
    /// `PATH2` can contain any non-whitespace character except `(`, `)`, `<` and `>`. It must end
    /// with a word-constituent character or any non-whitespace non-punctuation character followed
    /// by `/`.
    ///
    /// `PATH3` follows one of these patterns:
    ///
    /// - file type: `FILENAME`, which is a absolute or relative file path
    /// - protocol type: `PROTOCOL:PATH4` or `PROTOCOL://PATH4`
    /// - id type: `id:ID`, where `ID` is a hexadecimal number optionally separated by hyphens
    /// - custom-id type: `#CUSTOM-ID`
    /// - coderef type: `(CODEREF)`
    /// - fuzzy type: `FUZZY`
    ///
    /// And can be followed by double colons (`::`) and another string containing anything except
    /// `]`. Which will be used as the search option for following the link. See [`SearchOption`].
    ///
    /// `PATH4`, `CUSTOM-ID`, `CODEREF` and `FIZZY` can contain any character except square
    /// brackets.
    ///
    /// `DESCRIPTION` is optional and must be enclosed with square brackets. It can contain any
    /// character except square brackets. It is also parsed as a [`SecondaryString`] and can
    /// contain any object found in a paragraph except a [`FootnoteReference`], a [`RadioTarget`]
    /// and a [`LineBreak`]. It also can't contain another link unless it is a plain or angle link.
    ///
    /// Whitespace and newlines in the link are replaced with a single space.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Link {
        link: LinkFormat,
    }

    /// The format with the actual link data of a [`Link`].
    pub enum LinkFormat {
        Radio(ObjectId), // TODO only allow RadioTargets
        Angle(String),
        Plain(String),
        /// The secondary string can contain: export snippet, inline babel call, inline src block,
        /// latex fragment, entity, macro, plain link, statistics cookie, sub/superscript,
        /// text markup.
        Bracket(LinkPath, Option<SearchOption>, SecondaryString),
    }

    /// The kind and data of a bracket link in [`LinkFormat`].
    pub enum LinkPath {
        File(String),
        Id(String),
        CustomId(String),
        CodeRef(String),
        Fuzzy(String),
    }

    /// The search option of bracket [`LinkFormat`].
    pub enum SearchOption {
        /// Jump to line.
        Line(u64),
        /// Search for target (`<<TARGET>>`) or do a text search.
        Target(String),
        /// Restrict search to headlines.
        Headlines(String),
        /// Search for a custom id property (`:CUSTOM_ID: ...`).
        CustomId(String),
        /// Do a regular expression search.
        Regex(String),
    }

    /// A macro.
    ///
    /// # Semantics
    ///
    /// Macros are replaced by actual value when exporting.
    ///
    /// Replacement values are defined in the variabel `org-export-global-macros` or document wide
    /// with `#+MACRO: name     replacement text $1, $2 are arguments`. This macro can then be
    /// called with `{{{name(arg1, arg2)}}` resulting in `replacement text arg1, arg2 are
    /// arguments` when exporting.
    ///
    /// The following macros are pre-defined:
    ///
    /// - title
    /// - author
    /// - email
    /// - date(FORMAT): refers to the `#+DATE` keyword. (FORMAT is optional)
    /// - time(FORMAT): refers to the current date and time when exporting.
    /// - modification-time(FORMAT, VC): refers to the last modified attribute of the file on disk.
    ///   If VC is given (e.g. `true`) then try to retrieve the modifiaction time from a version
    ///   control system but falls back to file attributes.
    /// - input-file: refers to the filename of the exported file.
    /// - property(PROPERTY-NAME, SEARCH-OPTION): returns the PROPERTY-NAME in the current element.
    ///   If SEARCH-OPTION refers to a remote entry that will be used instead.
    /// - n(NAME, ACTION): Implements a custom counter by returning the number of times this macro
    ///   has been expanded so far. Using NAME creates different counters. If ACTION is `-` the
    ///   counter is not incremented. If ACTION is a number the counter is set to that value. If
    ///   ACTION is anything else the counter is reset to 1. You can reset the default timer by
    ///   leaving NAME empty.
    ///
    ///   TODO: None of these are implemented yet. Also exporting isn't implemented (and may never
    ///   be).
    ///
    /// # Syntax
    ///
    /// ```text
    /// {{{NAME(ARGUMENTS)}}}
    /// ```
    ///
    /// `NAME` must atart with a letter and can be followed by any number of alpha-numeric
    /// characters, hyphens and underscores.
    ///
    /// `ARGUMENTS` can contain anything but the string `}}}`. Arguments are separated by commas.
    /// Non-separating commas have to be escaped with a backslash character (if you want a literal
    /// backslash directly before the comma it has to be escaped with another backslash).
    ///
    /// Multiple whitespace and newline characters in `ARGUMENTS` are replaced by a single space.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Macro {
        name: String,
        arguments: Vec<String>,
    }

    /// A target that is automatically linked to.
    ///
    /// # Semantics
    ///
    /// A radio target e.g. with the value `<<<My Target>>>` makes every occurrence of the text `my
    /// target` (case is ignored) in the document link to the target.
    ///
    /// TODO I think this is only in the editor. Not sure how they are exported.
    ///
    /// # Syntax
    ///
    /// ```text
    /// <<<TARGET>>>
    /// ```
    ///
    /// `TARGET` can contain any character except `<`, `>` and newline. It can't start or end with
    /// a whitespace character. It will be parsed as a [`SecondaryString`].
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct RadioTarget {
        target: SecondaryString,
    }

    /// A statistics cookie.
    ///
    /// # Semantics
    ///
    /// TODO
    ///
    /// # Syntax
    ///
    /// There are two kinds of cookies:
    ///
    /// - percentage: `[PERCENT%]`
    /// - number: `[NUM1/NUM2]`
    ///
    /// `PERCENT`, `NUM1` and `NUM2` are numbers or an empty string.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct StatisticsCookie {
        cookie: CookieKind,
    }

    /// This is the kind and data of a [`StatisticsCookie`].
    pub enum CookieKind {
        Percent(Option<u32>),
        Number(Option<u32>, Option<u32>),
    }

    /// A subscript.
    ///
    /// # Semantics
    ///
    /// A subscript in the text.
    ///
    /// # Syntax
    ///
    /// ```text
    /// CHAR_SCRIPT
    /// ```
    ///
    /// `CHAR` is any non-whitespace character.
    ///
    /// `SCRIPT` can be `*` or any expression enclosed in parenthesis or curly brackets. It can
    /// contain balanced parenthesis and curly brackets.
    ///
    /// Or `SCRIPT` can collow the pattern:
    ///
    /// ```text
    /// SIGN CHARS FINAL
    /// ```
    ///
    /// `SIGN` is either a plus sign, a minus sign or an empty string.
    ///
    /// `CHARS` in any number of alpha-numeric characters, comas, backslashes and dots or an empty
    /// string.
    ///
    /// `FINAL` is an alpha-numeric character.
    ///
    /// There is no whitespace between `SIGN`, `CHARS` and `FINAL`.
    ///
    /// TODO this is recursive object. figure out how to handle recursive objects because some can
    /// only contain specific objects and therefore other recursive objects in them may contain
    /// less objects than they can usually contain
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Subscript {
        used_brackets: bool,
        // content: SecondaryString, // can contain the standard set of objects.
    }

    /// A superscript.
    ///
    /// # Semantics
    ///
    /// A superscript in the text.
    ///
    /// # Syntax
    ///
    /// ```text
    /// CHAR_SCRIPT
    /// ```
    ///
    /// See [`Subscript`].
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Superscript {
        used_brackets: bool,
        // content: SecondaryString, // can contain the standard set of objects.
    }

    /// A table cell in a [`greater_elements::TableRow`].
    ///
    /// # Semantics
    ///
    /// The content of a table row.
    ///
    /// # Syntax
    ///
    /// ```text
    /// CONTENTS SPACES |
    /// ```
    ///
    /// `CONTENTS` can contain any character except a vertical bar.
    ///
    /// `SPACES` contains any number (including zero) of soace and tab characters. This is usually
    /// used to align the table properly.
    ///
    /// The final bar my be replaced with a newline character for the last cell in the row.
    ///
    /// TODO recusrive object. can contain: export snippet, footnote reference, latex fragment,
    /// entity, link, macro, radio target, sub/superscript, target, text markup, timestamp
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct TableCell {
    }

    /// A target.
    ///
    /// # Semantics
    ///
    /// Used to link to internal objects that can't be assigned affiliated keywords. E.g. list
    /// items.
    ///
    /// See fuzzy [`Link`]s.
    ///
    /// # Syntax
    ///
    /// ```text
    /// <<TARGET>>
    /// ```
    ///
    /// `TARGET` can contain any character except `<`, `>` and newline. It can't start or end with
    /// a whitespace character. It will not be parsed.
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Target {
        target: String,
    }

    /// A text formatter.
    ///
    /// # Semantics
    ///
    /// Formats text according to the marker used:
    ///
    /// - `*bold*`
    /// - `/italic/`
    /// - `_underline_`
    /// - `+strike through+`
    /// - `~code~`
    /// - `=verbatim=`
    ///
    /// # Syntax
    ///
    /// ```text
    /// PRE MARKER BORDER BODY BORDER MARKER POST
    /// ```
    ///
    /// Not separated by any whitespace.
    ///
    /// `PRE` is one of `-`, whitespace, `(`, `'`,`"`, `{` or beginning of line.
    ///
    /// `BORDER` is anything but whitespace, `,`, `'` and `"`.
    ///
    /// `MARKER` is one of the markers specified in [semantics][#Semantics].
    ///
    /// `BODY` can contain any character but may not span over more than 3 lines.
    ///
    /// `POST` is one of `-`, whitespace, `.`, `,`, `:`, `!`, `?`, `;`, `'`, `"`, `)`, `}`, `[` or
    /// end of line.
    ///
    /// The part `BORDER BODY BORDER` is parsed as a [`SecondaryString`] and can contain the
    /// standard set of objects when the markup is bold, italic, strike through or udnerline. The
    /// content of verbatim and code is not parsed.
    ///
    /// TODO recursive object
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct TextMarkup {
        kind: TextMarkupKind,
    }

    /// The kind and content of a [`TextMarkup`] object.
    ///
    /// Only code and verbatim can't contain other objects.
    pub enum TextMarkupKind {
        Bold(SecondaryString),
        Italic(SecondaryString),
        Underline(SecondaryString),
        StrikeThrough(SecondaryString),
        Code(String),
        Verbatim(String),
    }

    pub use self::timestamp::Timestamp;

    /// Contains the [`Timestamp`][`timestamp::Timestamp`] object and all structs used by it.
    pub mod timestamp {
        use super::*;

        /// A timestamp.
        ///
        /// # Semantics
        ///
        /// Timestamps are used in [`elements::Clock`] and [`elements::Planning`] and can occur in normal text.
        ///
        /// They represent a date and time and can be either active or inactive. Usually inactive means
        /// that the event is already over or represents the date the event has been dealt with.
        ///
        /// # Syntax
        ///
        /// Follows one of the patterns:
        ///
        /// - diary sexp: `<%%(SEXP)>`
        /// - active: `<INNER>`
        /// - inactive: `[INNER]`
        /// - active range: `<INNER>--<INNER>` or `<DATE TIME-TIME REPEATERORDELAY>`
        /// - inactive range: `[INNER]--[INNER]` or `[DATE TIME-TIME REPEATERORDELAY]`
        ///
        /// `SEXP` can contain any character except `>` and newline.
        ///
        /// `INNER` is the pattern `DATE TIME REPEATERORDERLAY`.
        ///
        /// `DATE` follows the pattern `YYYY-MM-DD DAYNAME`. Where `Y`, `M` and `D` are digits
        /// (`0`-`9`). `DAYNAME` is optional and can contain any non-whitespace character except `+`,
        /// `-`, `]`, `>`, digits and newlines. Usually it is the three letter name of the weekday.
        ///
        /// `TIME` follows the pattern `HH:MM`. Where `H` and `M` are digits. The first `H` can be
        /// omitted.
        ///
        /// `REPEATERORDELAY` follows the pattern `MARK VALUE UNIT` where `MARK` is one of `+`, `++`,
        /// `.+`, `-` or `--` for the repeat or delay strategy. `VALUE` is a (positive) number. `UNIT`
        /// is one of `h`, `d`, `w`, `m` or `y`.
        ///
        /// There can be two `REPEATERORYEAR` in the timestamp. One as a repeater and on as a warning
        /// delay.
        #[derive(Object, getters)]
        #[add_fields_for(Object)]
        pub struct Timestamp {
            kind: TimestampKind,
        }

        impl Timestamp {
            pub fn timestamp_start(&self) -> Option<(&Date, Option<&Time>)> {
                use self::TimestampKind::*;
                use self::TimestampRange::*;

                match &self.kind {
                    DiarySexp(_) => None, // TODO maybe this does have a start
                    Active(TimestampData {date, time, ..}) => Some((&date, time.as_ref())),
                    Inactive(TimestampData {date, time, ..}) => Some((&date, time.as_ref())),
                    ActiveRange(TimeRange(TimestampDataWithTime {date, time, ..}, ..)) => Some((&date, Some(&time))),
                    ActiveRange(DateRange(TimestampData {date, time, ..}, ..)) => Some((&date, time.as_ref())),
                    InactiveRange(TimeRange(TimestampDataWithTime {date, time, ..}, ..)) => Some((&date, Some(&time))),
                    InactiveRange(DateRange(TimestampData {date, time, ..}, ..)) => Some((&date, time.as_ref())),
                }
            }
            pub fn timestamp_end(&self) -> Option<(&Date, Option<&Time>)> {
                use self::TimestampKind::*;
                use self::TimestampRange::*;

                match &self.kind {
                    DiarySexp(_) => None, // TODO maybe this does have an end
                    Active(TimestampData {date, time, ..}) => Some((&date, time.as_ref())),
                    Inactive(TimestampData {date, time, ..}) => Some((&date, time.as_ref())),
                    ActiveRange(TimeRange(TimestampDataWithTime {date, ..}, time)) => Some((&date, Some(&time))),
                    ActiveRange(DateRange(_, TimestampData {date, time, ..})) => Some((&date, time.as_ref())),
                    InactiveRange(TimeRange(TimestampDataWithTime {date, ..}, time)) => Some((&date, Some(&time))),
                    InactiveRange(DateRange(_, TimestampData {date, time, ..})) => Some((&date, time.as_ref())),
                }
            }
            pub fn repeater(&self) -> Option<&Repeater> {
                use self::TimestampKind::*;
                use self::TimestampRange::*;

                match &self.kind {
                    DiarySexp(_) => None, // TODO maybe this does have a repeater
                    Active(TimestampData {repeater, ..}) => repeater.as_ref(),
                    Inactive(TimestampData {repeater, ..}) => repeater.as_ref(),
                    ActiveRange(TimeRange(TimestampDataWithTime {repeater, ..}, _)) => repeater.as_ref(),
                    ActiveRange(DateRange(TimestampData {repeater, ..}, _)) => repeater.as_ref(),
                    InactiveRange(TimeRange(TimestampDataWithTime {repeater, ..}, _)) => repeater.as_ref(),
                    InactiveRange(DateRange(TimestampData {repeater, ..}, _)) => repeater.as_ref(),
                }
            }
            pub fn warning(&self) -> Option<&Warning> {
                use self::TimestampKind::*;
                use self::TimestampRange::*;

                match &self.kind {
                    DiarySexp(_) => None, // TODO maybe this does have a repeater
                    Active(TimestampData {warning, ..}) => warning.as_ref(),
                    Inactive(TimestampData {warning, ..}) => warning.as_ref(),
                    ActiveRange(TimeRange(TimestampDataWithTime {warning, ..}, _)) => warning.as_ref(),
                    ActiveRange(DateRange(TimestampData {warning, ..}, _)) => warning.as_ref(),
                    InactiveRange(TimeRange(TimestampDataWithTime {warning, ..}, _)) => warning.as_ref(),
                    InactiveRange(DateRange(TimestampData {warning, ..}, _)) => warning.as_ref(),
                }
            }
        }

        /// The kind and date for a [`Timestamp`].
        pub enum TimestampKind {
            DiarySexp(String),
            Active(TimestampData),
            Inactive(TimestampData),
            ActiveRange(TimestampRange),
            InactiveRange(TimestampRange),
        }

        /// The data for a [`TimestampKind`] with optional [`Time`].
        pub struct TimestampData {
            pub date: Date,
            pub time: Option<Time>,
            pub repeater: Option<Repeater>,
            pub warning: Option<Warning>,
        }

        /// A date.
        ///
        /// This is a wrapper around [`chrono::NaiveDate`].
        pub struct Date(NaiveDate);

        /// A time.
        ///
        /// This is a wrapper around [`chrono::NaiveTime`].
        pub struct Time(NaiveTime);

        /// The repeater of a timestamp.
        ///
        /// See [`TimestampData`] and [`TimestampDataWithTime`].
        pub struct Repeater {
            pub period: TimePeriod,
            pub strategy: RepeatStrategy,
        }

        /// The warning delay of a timestamp.
        ///
        /// See [`TimestampData`] and [`TimestampDataWithTime`].
        pub struct Warning {
            pub delay: TimePeriod,
            pub strategy: WarningStrategy,
        }

        /// The time period (with unit) of a [`Repeater`] or [`Warning`].
        pub struct TimePeriod {
            pub value: u32,
            pub unit: TimeUnit,
        }

        /// The strategy of a [`Repeater`].
        pub enum RepeatStrategy {
            /// Add the repeat duration to the task date once.
            Cumulative,
            /// Add the repeat duration to the task date until the date is in the
            /// future (but at leas once).
            CatchUp,
            /// Add the repeat duration to the current time.
            Restart,
        }

        /// The strategy of a [`Warning`].
        pub enum WarningStrategy {
            /// Warns for all (repeated) date. Represented as `-` in the org file.
            All,
            /// Warns only for the first date. Represented as `--` in the org file.
            First,
        }

        /// The unit of a [`TimePeriod`].
        pub enum TimeUnit {
            Year,
            Month,
            Week,
            Day,
            Hour,
        }

        /// The data for a timestamp range.
        ///
        /// See [`TimestampKind`].
        pub enum TimestampRange {
            /// `<DATE TIME-TIME REPEATER-OR-DELAY>` or
            /// `[DATE TIME-TIME REPEATER-OR-DELAY]`
            TimeRange(TimestampDataWithTime, Time),
            /// `<DATE TIME REPEATER-OR-DELAY>--<DATE TIME REPEATER-OR-DELAY>` or
            /// `[DATE TIME REPEATER-OR-DELAY]--[DATE TIME REPEATER-OR-DELAY]`
            DateRange(TimestampData, TimestampData),
        }

        /// The data for a timestamp with a time.
        ///
        /// See [`TimestampRange`].
        pub struct TimestampDataWithTime {
            pub date: Date,
            pub time: Time,
            pub repeater: Option<Repeater>,
            pub warning: Option<Warning>,
        }

    }

}
