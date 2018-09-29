use super::*;

/// A section.
///
/// # Semantics
///
/// This is a container for the content after a [`Headline`] or at the beginning of an org file
/// before the first headline.
///
/// # Syntax
///
/// Sections are context-free.
///
/// A section contains directly any (greater) element. Only a [`Headline`] can contain a
/// section. Also content before the first headline in a document belongs to a section.
///
/// A section ends at the beginning of the next headline or the end of the file.
#[derive(Element, HasContent, GreaterElement, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Section {
    shared_behavior_data: SharedBehaviorData,
    content_data: ContentData<ElementSet>,
}