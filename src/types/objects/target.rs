use super::*;

/// A target.
///
/// # Semantics
///
/// Used to link to internal objects that can't be assigned affiliated keywords. E.g. list
/// items.
///
/// See fuzzy [`Link`]s.
///
/// # Syntax
///
/// ```text
/// <<TARGET>>
/// ```
///
/// `TARGET` can contain any character except `<`, `>` and newline. It can't start or end with
/// a whitespace character. It will not be parsed.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    shared_behavior_data: SharedBehaviorData,
    pub target: String,
}

