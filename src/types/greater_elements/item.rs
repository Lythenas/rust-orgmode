use super::*;

/// An item in a list.
///
/// # Semantics
///
/// This is an item in a list.
///
/// # Syntax
///
/// ```text
/// BULLET [@COUNTER] [CHECKBOX] TAG CONTENT
/// ```
///
/// `BULLET` is either an asterisk, a hyphen, a plus sign (for unordered lists) or follows the
/// pattern `COUNTER.` or `COUNTER)` (for ordered lists). This is the only required part of
/// item. `BULLET` is always followed by a whitespace or end of line.
///
/// `COUNTER` is a number or a single letter.
///
/// `CHECKBOX` is either a single whitespace, a `X` or a hyphen.
///
/// `TAG` follows the pattern `TAG-TEXT ::` where `TAG-TEXT` can contain any character except a
/// newline. Only parsed as the description in unordered lists. Then the list is a description
/// list.
///
/// `CONTENT` is parsed as
///
/// An item ends before the next item, the first line that is less or equally indented that its
/// starting line or two consecutive empty lines. Indentation of lines within other greater
/// elements including inlinetask boundaries are ignored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    content: Spanned<Vec<StandardSetNoLineBreak>>,
    pub kind: ItemKind,
    pub checkbox: Option<Checkbox>,
    // structure ?
    // hiddenp: bool
}

impl Parent<Vec<StandardSetNoLineBreak>> for Item {
    fn content(&self) -> Option<&Spanned<Vec<StandardSetNoLineBreak>>> {
        Some(&self.content)
    }
}
/// The kind of an [`Item`] (and it's metadata).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Unordered {
        bullet: UnorderedBullet,
    },
    Ordered {
        bullet: OrderedBullet,
        counter: Counter,
    },
    Description {
        bullet: UnorderedBullet,
        tag: String,
    },
}

/// An unordered bullet of a lists [`ItemKind`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnorderedBullet {
    Minus,
    Plus,
    Star,
}

/// An ordered bullet of a lists [`ItemKind`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderedBullet {
    pub counter: Counter,
    pub delimiter: CounterDelimiter,
}

/// A counter of an ordered [`Item`].
///
/// See [`ItemKind`] and [`OrderedBullet`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Counter {
    Number(u64),
    Letter(char),
}

/// A delimiter after a [`Counter`] in an [`OrderedBullet`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CounterDelimiter {
    Period,
    Parenthesis,
}

/// Checkbox of an [`Item`] in a list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Checkbox {
    /// A space. (Empty checkbox)
    Unchecked,
    /// `X`. (Checked checkbox)
    Checked,
    /// `-`. (Some children of this list item are unchecked and some are checked)
    Partial,
}
