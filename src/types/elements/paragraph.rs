use super::*;

/// A paragraph.
///
/// # Semantics
///
/// A paragraph is a list of strings and objects ([`SecondaryString`]). Line breaks in the text
/// are ignored and only [`objects::LineBreak`] will be recognized as a line break.
///
/// # Syntax
///
/// Everything that is not another element is a paragraph. Empty lines and other elements end
/// paragraphs but all inner elements of the current paragraph must be closed first.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    /// The content of the paragraph.
    ///
    /// Newlines are ignored and are not stored here.
    ///
    /// TODO also store the ignored newlines somewhere/somehow.
    pub content: SecondaryString<StandardSet>,
}
