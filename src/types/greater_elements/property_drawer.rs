use super::*;

/// A property drawer.
///
/// # Semantics
///
/// A drawer associated with a [`Headline`]. It contains attributes of a headline.
///
/// # Syntax
///
/// Property drawers are context-free.
///
/// ```text
/// :PROPERTIES:
/// CONTENTS
/// :END:
/// ```
///
/// `CONTENTS` consists of zero or more [`elements::NodeProperty`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyDrawer {
    content: Spanned<Vec<elements::NodeProperty>>,
    // hiddenp: bool
}

impl Parent<Vec<elements::NodeProperty>> for PropertyDrawer {
    fn content(&self) -> Option<&Spanned<Vec<elements::NodeProperty>>> {
        Some(&self.content)
    }
}
