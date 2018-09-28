use super::*;

/// A table.
///
/// # Semantics
///
/// There are two types of tables:
///
/// - **org tables** can only contain [`TableRow`]s.
/// - **table.el tables** don't have parsed content.
///
/// # Syntax
///
/// Tables start with a line starting with a vertical bar or the string `+-` followed by plus
/// or binus signs only. Tables can be indented. The second line determines what type of table
/// this is.
///
/// # Org tables
///
/// Org tables start with a line starting with `|` and end at the first line not starting
/// with a vertical bar. They can be immediately followed by `#+TBLFM: FORMULAS` lines where
/// `FORMULAS` can contain any character.
///
/// ## Example
///
/// ```text
/// | col1 | col2 | col3 |
/// |------+------+------|
/// |  200 |  300 |  500 |
/// #+TBLFM: $3=$1+$2
/// ```
///
/// # Table.el tables
/// Table.el tables lines start with either a `|` or `+`. And end at the first line not
/// starting with either a vertical bar or a plus sign.
///
/// ## Example
///
/// ```text
/// +------+------+------+
/// | col1 | col2 | col3 |
/// +------+------+------+
/// |  200 |  300 |  500 |
/// +------+------+------+
/// ```
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Table {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: Spanned<AffiliatedKeywords>,
    // TODO make more type safe, org and table.el can't be mixed in one table.
    content_data: ContentData<TableContent>,
    pub kind: TableKind,
}

/// The set of objects that can be in a [`Table`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableContent {
    Org(TableRow),
    TableEl(String),
}

/// The kind of a [`Table`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableKind {
    Org,
    TableEl {
        formulas: Vec<String>,
        value: Option<String>,
    },
}
