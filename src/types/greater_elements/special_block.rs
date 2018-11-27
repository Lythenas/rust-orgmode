use crate::types::affiliated_keywords::AffiliatedKeywords;
use crate::types::{Element, GreaterElement, HasAffiliatedKeywords, Parent, Spanned};
use std::fmt;

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
/// `CONTENTS` can contain anything except a line `#+END_NAME` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// TODO not sure if this is actually a greater element
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecialBlock {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<String>,
    pub name: String,
    // hiddenp: bool
}
impl Parent<String> for SpecialBlock {
    fn content(&self) -> Option<&Spanned<String>> {
        Some(&self.content)
    }
}
impl HasAffiliatedKeywords for SpecialBlock {
    fn affiliated_keywords(&self) -> Option<&Spanned<AffiliatedKeywords>> {
        self.affiliated_keywords.as_ref()
    }
}
impl Element for SpecialBlock {}
impl GreaterElement for SpecialBlock {}

impl fmt::Display for SpecialBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#+BEGIN_{}", self.name)?;
        for line in self.content() {
            writeln!(f, "{}", line)?;
        }
        write!(f, "#+END_{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
