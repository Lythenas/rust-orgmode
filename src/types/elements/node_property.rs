/// A node property.
///
/// # Semantics
///
/// A property contained in a [`greater_elements::PropertyDrawer`].
///
/// # Syntax
///
/// Follows one of these patterns:
///
/// - `:NAME: VALUE`
/// - `:NAME+: VALUE`
/// - `:NAME:`
/// - `:NAME+:`
///
/// `NAME` can contain any non-whitespace character but can't be an empty string or end with a
/// plus sign (`+`).
///
/// `VALUE` can contain anything but a newline character.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeProperty {
    pub name: String,
    pub value: String,
}
