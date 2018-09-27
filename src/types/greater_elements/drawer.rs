use super::*;

/// A drawer to hide content.
///
/// # Semantics
///
/// Used to hide content in the editor and when exporting. Drawers can usually be opened and
/// closed in the editor.
///
/// # Syntax
///
/// ```text
/// :NAME:
/// CONTENTS
/// :END:
/// ```
///
/// `NAME` can contain any word-constituent characters, hyphens and underscores.
///
/// `CONTENTS` can contain any element except a [`Headline`] and another drawer.
///
/// Drawers can be indented.
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Drawer {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    content_data: ContentData<ElementSet>,
    pub name: String,
    // hiddenp: bool,
}

fn _parse() {
    // search for "^[ \t]*:END:[ \t]*$"
    // if not found: parse a paragraph
    // if found: get end line position
    // parse the start line
    // check if has content
    // (if so: parse conten)
    // count blank lines after end (skip forward all " \r\t\n" chars)
}
