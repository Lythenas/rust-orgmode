use super::*;

/// A keyword.
///
/// # Semantics
///
/// A keywords is similar to [`AffiliatedKeywords`] but they don't belong to another element.
/// Orphaned affiliated keywords are considered regular keywords.
///
/// # Syntax
///
/// ```text
/// #+KEY: VALUE
/// ```
///
/// `KEY` can contain any non-whitespace character. But it can't be equal to `CALL` or any
/// affiliated keyword.
///
/// `VALUE` can contain any character except a newline.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keyword {
    affiliated_keywords: Spanned<AffiliatedKeywords>,
    pub key: String,
    pub value: Spanned<Vec<KeywordValueSetOfObjects>>,
}
impl Element for Keyword {}
impl HasAffiliatedKeywords for Keyword {
    fn affiliated_keywords(&self) -> Option<&Spanned<AffiliatedKeywords>> {
        Some(&self.affiliated_keywords)
    }
}

/// The set of objects a [`Keyword`] can contain.
///
/// Keywords can't contain [`objects::FootnoteReference`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordValueSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    ExportSnippet(objects::ExportSnippet),
    InlineBabelCall(objects::InlineBabelCall),
    InlineSrcBlock(objects::InlineSrcBlock),
    LatexFragment(objects::LatexFragment),
    LineBreak(objects::LineBreak),
    Link(objects::Link),
    Macro(objects::Macro),
    RadioTarget(objects::RadioTarget),
    StatisticsCookie(objects::StatisticsCookie),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    Target(objects::Target),
    TextMarkup(objects::TextMarkup),
    Timestamp(objects::Timestamp),
}

impl AsRawString for KeywordValueSetOfObjects {
    fn as_raw_string(&self) -> Option<&str> {
        if let KeywordValueSetOfObjects::RawString(s) = self {
            Some(s)
        } else {
            None
        }
    }
}
