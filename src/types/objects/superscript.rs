use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Superscript {
    pub used_brackets: bool,
    pub content: SecondaryString<StandardSet>,
}
