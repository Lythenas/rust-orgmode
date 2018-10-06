use super::*;

/// A quote.
///
/// # Semantics
///
/// Used for quotes. When exporting this block will be indented on the left and right margin.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_QUOTE
/// CONTENTS
/// #+END_QUOTE
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// TODO not sure if this is actually a greater element
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuoteBlock {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<Vec<ElementSet>>,
    // hiddenp: bool
}

impl Parent<Vec<ElementSet>> for QuoteBlock {
    fn content(&self) -> Option<&Spanned<Vec<ElementSet>>> {
        Some(&self.content)
    }
}
