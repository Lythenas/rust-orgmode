use super::*;

/// A table cell in a [`greater_elements::TableRow`].
///
/// # Semantics
///
/// The content of a table row.
///
/// # Syntax
///
/// ```text
/// CONTENTS SPACES |
/// ```
///
/// `CONTENTS` can contain any character except a vertical bar.
///
/// `SPACES` contains any number (including zero) of soace and tab characters. This is usually
/// used to align the table properly.
///
/// The final bar my be replaced with a newline character for the last cell in the row.
///
/// TODO recusrive object. can contain: export snippet, footnote reference, latex fragment,
/// entity, link, macro, radio target, sub/superscript, target, text markup, timestamp
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableCell {
    pub content: Spanned<TableCellSetOfObjects>,
}

impl Object for TableCell {}

/// The set of objects [`TableCell`] can contain.
///
/// Table cells can't contain [`InlineBabelCall`], [`InlineSrcBlock`] because formulas are
/// possible. Also they can't contain [`LineBreak`] and [`StatisticsCookie`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableCellSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    ExportSnippet(objects::ExportSnippet),
    FootnoteReference(objects::FootnoteReference),
    LatexFragment(objects::LatexFragment),
    Link(objects::Link),
    Macro(objects::Macro),
    RadioTarget(objects::RadioTarget),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    Target(objects::Target),
    TextMarkup(objects::TextMarkup),
    Timestamp(objects::Timestamp),
}

impl AsRawString for TableCellSetOfObjects {
    fn as_raw_string(&self) -> Option<&str> {
        if let TableCellSetOfObjects::RawString(s) = self {
            Some(s)
        } else {
            None
        }
    }
}
