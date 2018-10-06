use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InlineSrcBlock {
    pub lang: String,
    pub value: String,
    pub options: String,
}
