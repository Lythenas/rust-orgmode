use super::*;

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
