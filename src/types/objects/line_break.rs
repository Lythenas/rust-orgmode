use super::*;

/// A line break.
///
/// # Semantics
///
/// Used to export a line break.
///
/// # Syntax
///
/// ```text
/// \\SPACE
/// ```
///
/// `SPACE` is zero or more whitespace characters followed by the end of line or end of
/// document.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineBreak {
    shared_behavior_data: SharedBehaviorData,
    pub spaces: u64,
}
