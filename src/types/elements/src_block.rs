use super::*;

/// A block of source code.
///
/// # Semantics
///
/// Same as [`ExampleBlock`] but usually contains source code. The content will be highlighted
/// according to the language specified.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_SRC LANGUAGE FLAGS ARGUMENTS
/// CONTENTS
/// #+END_SRC
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_SRC` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// `LANGUAGE` can contain anything except whitespace.
///
/// `FLAGS` see [`BlockFlags`].
///
/// `ARGUMENTS` can contain any character except a newline.
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SrcBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    pub language: String,
    pub flags: BlockFlags,
    pub arguments: String,
}
