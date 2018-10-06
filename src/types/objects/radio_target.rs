use super::*;

/// A target that is automatically linked to.
///
/// # Semantics
///
/// A radio target e.g. with the value `<<<My Target>>>` makes every occurrence of the text `my
/// target` (case is ignored) in the document link to the target.
///
/// TODO I think this is only in the editor. Not sure how they are exported.
///
/// # Syntax
///
/// ```text
/// <<<TARGET>>>
/// ```
///
/// `TARGET` can contain any character except `<`, `>` and newline. It can't start or end with
/// a whitespace character. It will be parsed as a [`SecondaryString`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RadioTarget {
    pub target: SecondaryString<StandardSet>,
}

/// The set of objects a [`RadioTarget`] can contain.
///
/// Radio targets content is limited to easy to parse objects. Mainly just markup and raw strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RadioTargetSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    LatexFragment(objects::LatexFragment),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    TextMarkup(objects::TextMarkup),
}

impl AsRawString for RadioTargetSetOfObjects {
    fn as_raw_string(&self) -> Option<&str> {
        if let RadioTargetSetOfObjects::RawString(s) = self {
            Some(s)
        } else {
            None
        }
    }
}
