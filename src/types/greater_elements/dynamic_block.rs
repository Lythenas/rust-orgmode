use super::*;

/// A dynamic block.
///
/// # Semantics
///
/// The content of dynamic blocks can be updated automatically by calling a function with
/// the given name and parameters. If that function needs the previous content of the block an
/// extra parameter `:content` has to be added.
///
/// # Syntax
///
/// ```text
/// #+BEGIN: NAME PARAMETERS
/// CONTENTS
/// #+END:
/// ```
///
/// Note the `:` after `BEGIN` and `END`. It can be omitted after `END` without generating an
/// error.
///
/// `NAME` can contain anything except whitespace.
///
/// `PARAMETERS` can contain any character and can be omitted. They are usually of the format
/// `:name value` or `:name`.
///
/// `CONTENTS` is auto-generated and will not be parsed.
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct DynamicBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: AffiliatedKeywordsData,
    content_data: ContentData<ElementSet>,
    /// The name of the function that can update this block.
    pub name: String,
    /// The parameters to pass to the function updating this block.
    ///
    /// Usually of the format `:name value`, separated by a space. Value can also be omitted.
    ///
    /// If the function needs the current content of the block add a parameter `:content`.
    pub parameters: String, // TODO maybe parse this as a list
                            // hiddenp: bool
}
