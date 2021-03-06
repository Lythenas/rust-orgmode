use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    pub value: String,
}
