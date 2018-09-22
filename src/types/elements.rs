//! Contains all elements except [`greater_elements`].

use super::*;

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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BabelCall {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    /// The code block to call
    pub call: String,
    pub inside_header: String,
    pub arguments: String,
    pub end_header: String,
}

/// A clock element.
///
/// # Sematics
///
/// A clock element is used to track working time. When clocked in the timestamp part is only a
/// date and time. When clocked out the timestamp part is a datetime range. And the duration is
/// the duration of the range.
///
/// The timestamps are usually inactive.
///
/// # Syntax
///
/// ```text
/// CLOCK: TIMESTAMP DURATION
/// ```
///
/// `CLOCK` is the literal word `CLOCK`.
///
/// `TIMESTAMP` and `DURATION` are optional. `TIMESTAMP` is a [`objects::Timestamp`].
///
/// `DURATION` follows the pattern `=> HH:MM` where `HH` is a number containing any number of
/// digits and `MM` is a two digit number.
#[derive(Element, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clock {
    shared_behavior_data: SharedBehaviorData,
    pub timestamp: Option<objects::Timestamp>,
    pub duration: Option<(u64, u8)>,
}

impl Clock {
    pub fn status(&self) -> ClockStatus {
        match self.duration {
            Some(_) => ClockStatus::Closed,
            None => ClockStatus::Running,
        }
    }
}

/// The status of a [`Clock`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub value: String,
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub value: String,
}

/// A diary sexp.
///
/// # Semantics
///
/// Diary sexps are special function to related to date and time. E.g. you can automatically
/// calculate the age of someone by giving it a birthday. It can also display all holidays.
///
/// See <https://orgmode.org/manual/Weekly_002fdaily-agenda.html> for more info.
///
/// # Syntax
///
/// ```text
/// %%(VALUE
/// ```
///
/// `VALUE` can contain any character except a newline. The expression has to start at the
/// beginning of the line.
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiarySexp {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub value: String,
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExampleBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub value: String,
    pub flags: BlockFlags,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockFlags {
    pub number_lines: Option<NumberLinesFlag>,
    /// Default: false
    pub preserve_indent: bool,
    /// Default: true
    ///
    /// If true, code-references should use labels instead of line numbers.
    pub retain_labels: bool,
    pub label_fmt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumberLinesFlag {
    Continued(Option<u64>),
    New(Option<u64>),
}

/// An export block.
///
/// # Semantics
///
/// This block will only be exported in the specified backend.
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub value: String,
    /// Always lowercase.
    pub backend: String,
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedWidth {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub value: String,
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HorizontalRule {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keyword {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub key: String,
    pub value: ContentData<KeywordValueSetOfObjects>,
}

#[derive(AsRawString, Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordValueSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    ExportSnippet(objects::ExportSnippet),
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LatexEnvironment {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    /// Contains everything including `\begin...` and `\end`.
    pub value: String,
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
#[derive(Element, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeProperty {
    shared_behavior_data: SharedBehaviorData,
    pub name: String,
    pub value: String,
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
/// Everything that is not another element is a paragraph. Empty lines and other elements end
/// paragraphs but all inner elements of the current paragraph must be closed first.
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    /// The content of the paragraph.
    ///
    /// Newlines are ignored and are not stored here.
    ///
    /// TODO also store the ignored newlines somewhere/somehow.
    pub content: SecondaryString<StandardSet>,
}

/// A planning element.
///
/// # Semantics
///
/// Contains the deadline, scheduled and closed timestamps for a headline. All are optional.
///
/// # Syntax
///
/// Planning lines are context-free.
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
#[derive(Element, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Planning {
    shared_behavior_data: SharedBehaviorData,
    pub closed: Option<objects::Timestamp>,
    pub deadline: Option<objects::Timestamp>,
    pub scheduled: Option<objects::Timestamp>,
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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SrcBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub language: String,
    pub flags: BlockFlags,
    pub arguments: String,
}
