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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerseBlock {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<Vec<StandardSet>>,
}

impl Parent<Vec<StandardSet>> for VerseBlock {
    fn content(&self) -> Option<&Spanned<Vec<StandardSet>>> {
        Some(&self.content)
    }
}
