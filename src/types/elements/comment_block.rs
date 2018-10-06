use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentBlock {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    pub value: String,
}
