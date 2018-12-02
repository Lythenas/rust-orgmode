use crate::types::{
    AffiliatedKeywords, Element, ElementSet, GreaterElement, HasAffiliatedKeywords, Parent, Spanned,
};
use std::fmt;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Drawer {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<Vec<ElementSet>>,
    pub name: String,
    // hiddenp: bool,
}

impl Parent<Vec<ElementSet>> for Drawer {
    fn content(&self) -> Option<&Spanned<Vec<ElementSet>>> {
        Some(&self.content)
    }
}

impl Element for Drawer {}
impl GreaterElement for Drawer {}
impl HasAffiliatedKeywords for Drawer {
    fn affiliated_keywords(&self) -> Option<&Spanned<AffiliatedKeywords>> {
        self.affiliated_keywords.as_ref()
    }
}

impl fmt::Display for Drawer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ":{}:", self.name)?;
        for _line in self.content() {
            // TODO this should work once all elements impl Display (also impl Display for ElementSet)
            //writeln!(f, "{}", line)?;
        }
        write!(f, ":END:")
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
}
