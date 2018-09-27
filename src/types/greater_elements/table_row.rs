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
#[derive(Element, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableRow {
    shared_behavior_data: SharedBehaviorData,
    pub kind: TableRowKind,
}

impl GreaterElement<objects::TableCell> for TableRow {}
impl HasContent<objects::TableCell> for TableRow {
    fn content_data(&self) -> &ContentData<objects::TableCell> {
        match self.kind {
            TableRowKind::Normal(ref content) => &content,
            TableRowKind::Rule => &EMPTY_CONTENT_DATA_FOR_TABLE_ROWS,
        }
    }
}

static EMPTY_CONTENT_DATA_FOR_TABLE_ROWS: ContentData<objects::TableCell> = ContentData {
    span: Span { start: 0, end: 0 },
    content: Vec::new(),
};

/// The kind of a [`TableRow`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableRowKind {
    Normal(ContentData<objects::TableCell>),
    Rule,
}
