use super::*;

/// A center block.
///
/// # Semantics
///
/// Centers text. Also the content can contain markup.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_CENTER
/// CONTENTS
/// #+END_CENTER
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct CenterBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: Spanned<AffiliatedKeywords>,
    content_data: ContentData<String>,
}
