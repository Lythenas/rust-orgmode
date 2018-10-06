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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CenterBlock {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<String>,
}

impl Parent<String> for CenterBlock {
    fn content(&self) -> Option<&Spanned<String>> {
        Some(&self.content)
    }
}
