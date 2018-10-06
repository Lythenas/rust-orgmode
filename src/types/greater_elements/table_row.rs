use super::*;

/// A row in a [`Table`][`Table`].
///
/// # Semantics
///
/// A row contains cell which can contain content.
///
/// # Syntax
///
/// There are two kinds of table rows:
///
/// - normal: vertical bar and any number of [`TableCell`][`objects::TableCell`]s
///   ```text
///   | cell 1 | cell 2 | ... |
///   ```
/// - a rule: vertical bar followed by hyphens followed by a vertical bar
///   ```text
///   |--------|
///   ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableRow {
    pub kind: TableRowKind,
}

impl Element for TableRow {}
impl GreaterElement for TableRow {}
impl Parent<Vec<objects::TableCell>> for TableRow {
    fn content(&self) -> Option<&Spanned<Vec<objects::TableCell>>> {
        match self.kind {
            TableRowKind::Normal(ref content) => Some(&content),
            TableRowKind::Rule => None,
        }
    }
}

/// The kind of a [`TableRow`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableRowKind {
    Normal(Spanned<Vec<objects::TableCell>>),
    Rule,
}
