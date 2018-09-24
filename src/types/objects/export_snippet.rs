use super::*;

/// An export snippet.
///
/// # Semantics
///
/// These snippets are only exported in the specified format. E.g. there can be an export
/// snippet that is only exported in html.
///
/// # Syntax
///
/// ```text
/// @@BACKEND:VALUE@@
/// ```
///
/// `BACKEND` can contain any alpha-numeric character and hyphens.
///
/// `VALUE` can contain anything but the `@@` string.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportSnippet {
    shared_behavior_data: SharedBehaviorData,
    pub backend: String,
    pub value: String,
}
