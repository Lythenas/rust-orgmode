use super::*;

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
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextMarkup {
    shared_behavior_data: SharedBehaviorData,
    pub kind: TextMarkupKind,
}

/// The kind and content of a [`TextMarkup`] object.
///
/// Only code and verbatim can't contain other objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextMarkupKind {
    // TODO maybe make these actual different types instead of an enum
    Bold(SecondaryString<StandardSet>),
    Italic(SecondaryString<StandardSet>),
    Underline(SecondaryString<StandardSet>),
    StrikeThrough(SecondaryString<StandardSet>),
    Code(String),
    Verbatim(String),
}
