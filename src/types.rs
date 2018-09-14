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
        Continued(u64),
        New(u64),
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
    }

    // TODO DocumentProperty same as keyword but contains objects.

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
        keyword: Option<String>, // TODO mybe make separate enum
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

    /// An inline task.
    ///
    /// # Semantics
    ///
    /// TODO
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
        keyword: Option<String>, // TODO mybe make separate enum
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
        bullet: String, // TODO make struct
        checkbox: Option<Checkbox>,
        counter: String, // TODO make struct
        tag: String,
        // structure ?
        // hiddenp: bool
    }

    // TODO find better names for the variants
    pub enum Checkbox {
        /// A space.
        Off,
        /// A `X`.
        X,
        /// A `-`.
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
        kind: ListKind,
        // structure ?
    }

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

    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Entity {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct ExportSnippet {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct FootnoteReference {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct InlineBabelCall {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct InlineSrcBlock {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct LatexFragment {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct LineBreak {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Link {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Macro {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct RadioTarget {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct StatisticsCookie {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Subscript {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Superscript {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct TableCell {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Target {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Bold {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Italic {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Underline {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct StrikeThrough {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Code {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Verbatim {
    }
    #[derive(Object, getters)]
    #[add_fields_for(Object)]
    pub struct Timestamp {
    }

}
