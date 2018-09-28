use super::*;

/// A babel call element.
///
/// # Sematics
///
/// Used to execute [`SrcBlock`]s and put their results into the org file.
///
/// # Syntax
///
/// ```text
/// #+CALL: FUNCTION[INSIDE-HEADER](ARGUMENTS) END-HEADER
/// ```
///
/// `FUNCTION` is the name of a [`SrcBlock`] to execute. `INSIDE-HEADER`, `ARGUEMENTS` and
/// `END-HEADER` can contain everything except a newline (and their respective closing char).
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BabelCall {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: Spanned<AffiliatedKeywords>,
    /// The code block to call
    pub call: String,
    pub inside_header: String,
    pub arguments: String,
    pub end_header: String,
}
