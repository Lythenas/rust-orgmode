use super::*;

/// An export block.
///
/// # Semantics
///
/// This block will only be exported in the specified backend.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_EXPORT BACKEND
/// CONTENTS
/// #+END_EXPORT
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_EXAMPLE` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// `BACKEND` can contain any alpha-numerical character. Case is ignored.
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub value: String,
    /// Always lowercase.
    pub backend: String,
}
