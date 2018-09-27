//! Contains all elements except [`greater_elements`].

use super::*;

mod babel_call;
mod clock;
mod comment_block;
mod comment;
mod diary_sexp;
mod example_block;
mod export_block;
mod fixed_width;
mod horizontal_rule;
mod keyword;
mod latex_environment;
mod node_property;
mod paragraph;
mod planning;
mod src_block;

pub use self::babel_call::BabelCall;
pub use self::clock::{Clock, ClockStatus};
pub use self::comment_block::CommentBlock;
pub use self::comment::Comment;
pub use self::diary_sexp::DiarySexp;
pub use self::example_block::ExampleBlock;
pub use self::export_block::ExportBlock;
pub use self::fixed_width::FixedWidth;
pub use self::horizontal_rule::HorizontalRule;
pub use self::keyword::{Keyword, KeywordValueSetOfObjects};
pub use self::latex_environment::LatexEnvironment;
pub use self::node_property::NodeProperty;
pub use self::paragraph::Paragraph;
pub use self::planning::Planning;
pub use self::src_block::SrcBlock;

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

/// Flag of [`BlockFlags`] that defines if line numbering is continued or start fresh (and
/// optionally from where)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumberLinesFlag {
    Continued(Option<u64>),
    New(Option<u64>),
}

