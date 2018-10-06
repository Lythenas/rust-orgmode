use super::*;

/// A statistics cookie.
///
/// # Semantics
///
/// Used in [`Headline`]s, [`Inlinetask`]s and the first [`Item`] of [`PlainList`]s to represent
/// the amount of done tasks or checked items.
///
/// # Syntax
///
/// There are two kinds of cookies:
///
/// - percentage: `[PERCENT%]`
/// - number: `[NUM1/NUM2]`
///
/// `PERCENT`, `NUM1` and `NUM2` are numbers or an empty string.
///
/// [`Headline`]: `greater_elements::Headline`
/// [`Inlinetask`]: `greater_elements::Inlinetask`
/// [`Item`]: `greater_elements::Item`
/// [`PlainList`]: `greater_elements::PlainList`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatisticsCookie {
    pub cookie: CookieKind,
}

/// This is the kind and data of a [`StatisticsCookie`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CookieKind {
    Percent(Option<u32>),
    Number(Option<u32>, Option<u32>),
}
