use super::*;

/// A footnote definition.
///
/// # Semantics
///
/// Defines a footnote that can be references with a [`objects::FootnoteReference`].
///
/// # Syntax
///
/// ```text
/// [LABEL] CONTENTS
/// ```
///
/// `LABEL` is either a number or follows the pattern `fn:WORD` where `WORD` can contain any
/// word-constituent character, hyphens and underscores.
///
/// `CONTENTS` can contain any element except another footnote definition and a [`Headline`].
/// It ends at the next footnote definition, headline, with two consecutive empty lines or the
/// end of the buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FootnoteDefinition {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<Vec<ElementSet>>, // TODO
    pub label: String,
    // pre_blank: u32 // TODO (maybe) blank lines after `[LABEL]`
}

impl Parent<Vec<ElementSet>> for FootnoteDefinition {
    fn content(&self) -> Option<&Spanned<Vec<ElementSet>>> {
        Some(&self.content)
    }
}
