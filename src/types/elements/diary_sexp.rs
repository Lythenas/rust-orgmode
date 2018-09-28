use super::*;

/// A diary sexp.
///
/// # Semantics
///
/// Diary sexps are special function to related to date and time. E.g. you can automatically
/// calculate the age of someone by giving it a birthday. It can also display all holidays.
///
/// See <https://orgmode.org/manual/Weekly_002fdaily-agenda.html> for more info.
///
/// # Syntax
///
/// ```text
/// %%(VALUE
/// ```
///
/// `VALUE` can contain any character except a newline. The expression has to start at the
/// beginning of the line.
#[derive(Element, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiarySexp {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: Spanned<AffiliatedKeywords>,
    pub value: String,
}
