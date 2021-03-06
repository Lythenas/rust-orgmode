use super::*;

/// A plain list.
///
/// # Semantics
///
/// A complete list of [`Item`]s.
///
/// # Syntax
///
/// This is a set of consecutive items of the same indentation. It can only directly contain
/// items.
///
/// If the dirst item has a `COUNTER` in its `BULLET` the plain list is be an *ordered plain
/// list*. If it contains a tag it is be a *descriptive list*. Otherwise it is be an
/// *unordered list*.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlainList {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<Vec<Item>>,
    // structure ?
}

impl Parent<Vec<Item>> for PlainList {
    fn content(&self) -> Option<&Spanned<Vec<Item>>> {
        Some(&self.content)
    }
}

impl PlainList {
    pub fn kind(&self) -> ListKind {
        // find first item and get kind of item
        // TODO not sure if this is the best way
        unimplemented!()
    }
}

/// The list kind of a [`PlainList`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListKind {
    Unordered,
    Ordered,
    Description,
}
