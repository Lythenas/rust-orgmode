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
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RadioTarget {
    shared_behavior_data: SharedBehaviorData,
    pub target: SecondaryString<StandardSet>,
}

#[derive(AsRawString, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RadioTargetSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    LatexFragment(objects::LatexFragment),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    TextMarkup(objects::TextMarkup),
}
