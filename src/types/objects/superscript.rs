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
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Superscript {
    shared_behavior_data: SharedBehaviorData,
    pub used_brackets: bool,
    pub content: SecondaryString<StandardSet>,
}

