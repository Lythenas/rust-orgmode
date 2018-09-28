use super::*;
use regex::{self, Regex};
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
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Drawer {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    content_data: ContentData<ElementSet>,
    pub name: String,
    // hiddenp: bool,
}

lazy_static! {
    static ref RE_START: Regex =
        Regex::new(r"(?m)\A^(?P<indentation>\s*):(?P<name>[\w-_]+):\s*$").unwrap();
    static ref RE_END: Regex = Regex::new(r"(?m)\A^(?P<indentation>\s*):END:\s*$").unwrap();
}

impl fmt::Display for Drawer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO this should work once all elements impl Display (also impl Display for ElementSet)
        //write!(f, ":{}:\n{}\n:END:", self.name, self.content_data)
        unimplemented!()
    }
}
