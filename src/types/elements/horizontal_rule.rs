use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HorizontalRule {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
}
