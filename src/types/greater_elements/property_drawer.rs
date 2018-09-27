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
#[derive(Element, HasContent, GreaterElement, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyDrawer {
    shared_behavior_data: SharedBehaviorData,
    content_data: ContentData<elements::NodeProperty>,
    // hiddenp: bool
}
