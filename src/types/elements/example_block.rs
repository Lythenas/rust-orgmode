use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExampleBlock {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    pub value: String,
    pub flags: BlockFlags,
}
