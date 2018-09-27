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
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HorizontalRule {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
}
