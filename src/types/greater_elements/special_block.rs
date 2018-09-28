use super::*;

/// A special block.
///
/// # Semantics
///
/// Any block with name that is not recognized as another block is a special block.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_NAME
/// CONTENTS
/// #+END_NAME
/// ```
///
/// `NAME` can contain any non-whitespace character.
///
/// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// TODO not sure if this is actually a greater element
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct SpecialBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: Spanned<AffiliatedKeywords>,
    content_data: ContentData<String>,
    pub kind: String,
    // hiddenp: bool
}
