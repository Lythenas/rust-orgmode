use super::*;

/// A verse block.
///
/// # Semantics
///
/// Simmilar to an [`elements::ExampleBlock`] but content is interpreted as objects. Verse blocks
/// preserve indentation.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_VERSE
/// CONTENTS
/// #+END_VERSE
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_VERSE` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will be parsed as objects.
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct VerseBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    content_data: ContentData<StandardSet>,
}
