//! `AffiliatedKeywords` holds the attributes affiliated with an element.

use super::*;
use std::slice;

/// Contains all affiliated keywords for one element/object.
///
/// An affiliated keyword represents an attribute of an element.
///
/// Not all elements can have affiliated keywords. See the specific element.
///
/// Affiliated keywords have one of the following formats:
///
/// - `#+KEY: VALUE`
/// - `#+KEY[OPTIONAL]: VALUE`
/// - `#+ATTR_BACKEND: VALUE`
///
/// # Captions
///
/// Parsed from: `#+CAPTION[OPTIONAL]: VALUE`.
///
/// Where `OPTIONAL` (and the brackets) are optional and both `OPTIONAL` and `VALUE` are
/// secondary strings (can contain objects).
///
/// The caption key can occur more than once.
///
/// # Headers
///
/// Parsed from: `#+HEADER: VALUE`.
///
/// The header key can occur more than once.
///
/// The deprecated `HEADERS` key will also be parsed to this variant.
///
/// # Name
///
/// Parsed from: `#+NAME: VALUE`.
///
/// The deprecated `LABEL`, `SRCNAME`, `TBLNAME`, `DATA`, `RESNAME` and `SOURCE` keys will also
/// be parsed to this variant.
///
/// # Plot
///
/// Parsed from: `#+PLOT: VALUE`.
///
/// # Result
///
/// Parsed from: `#+RESULTS[OPTIONAL]: VALUE`.
///
/// Where `OPTIONAL` (and the brackets) are optional and both `OPTIONAL` and `VALUE` are
/// secondary strings (can contain objects).
///
/// The deprecated `RESULT` key will also be parsed to this variant.
///
/// # Attrs
///
/// Parsed from: `#+ATTR_BACKEND: VALUE`.
///
/// The attr keywords for one backend can occur more than once.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AffiliatedKeywords {
    captions: Vec<Spanned<Caption>>,
    headers: Vec<Spanned<String>>,
    name: Option<Spanned<String>>,
    plot: Option<Spanned<String>>,
    results: Option<Spanned<Results>>,
    attrs: Vec<Spanned<Attr>>,
}

pub enum AffiliatedKeyword {
    Caption(Spanned<Caption>),
    Header(Spanned<String>),
    Name(Spanned<String>),
    Plot(Spanned<String>),
    Results(Spanned<Results>),
    Attr(Spanned<Attr>),
}

impl<A> std::iter::FromIterator<A> for AffiliatedKeywords
where
    A: Into<AffiliatedKeyword>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let mut ak = AffiliatedKeywords::default();
        ak.extend(iter);
        ak
    }
}

impl<A> Extend<A> for AffiliatedKeywords
where
    A: Into<AffiliatedKeyword>,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>,
    {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl AffiliatedKeywords {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, other: impl Into<AffiliatedKeyword>) -> Option<AffiliatedKeyword> {
        match other.into() {
            AffiliatedKeyword::Caption(caption) => {
                self.captions.push(caption);
                None
            }
            AffiliatedKeyword::Header(header) => {
                self.headers.push(header);
                None
            }
            AffiliatedKeyword::Name(name) => {
                self.name.replace(name).map(|x| AffiliatedKeyword::Name(x))
            }
            AffiliatedKeyword::Plot(plot) => {
                self.plot.replace(plot).map(|x| AffiliatedKeyword::Plot(x))
            }
            AffiliatedKeyword::Results(results) => self
                .results
                .replace(results)
                .map(|x| AffiliatedKeyword::Results(x)),
            AffiliatedKeyword::Attr(attr) => {
                self.attrs.push(attr);
                None
            }
        }
    }

    pub fn captions(&self) -> Captions<'_> {
        Captions {
            inner: self.captions.iter(),
        }
    }
    pub fn spanned_captions(&self) -> SpannedCaptions<'_> {
        SpannedCaptions {
            inner: self.captions.iter(),
        }
    }
    pub fn headers(&self) -> Headers<'_> {
        Headers {
            inner: self.headers.iter(),
        }
    }
    pub fn spanned_headers(&self) -> SpannedHeaders<'_> {
        SpannedHeaders {
            inner: self.headers.iter(),
        }
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref().map(|spanned| spanned.value())
    }
    pub fn spanned_name(&self) -> Option<&Spanned<String>> {
        self.name.as_ref()
    }
    pub fn plot(&self) -> Option<&String> {
        self.plot.as_ref().map(|spanned| spanned.value())
    }
    pub fn spanned_plot(&self) -> Option<&Spanned<String>> {
        self.plot.as_ref()
    }
    pub fn results(&self) -> Option<&Results> {
        self.results.as_ref().map(|spanned| spanned.value())
    }
    pub fn spanned_results(&self) -> Option<&Spanned<Results>> {
        self.results.as_ref()
    }
    pub fn attrs(&self) -> Attrs<'_> {
        Attrs {
            inner: self.attrs.iter(),
        }
    }
    pub fn spanned_attrs(&self) -> SpannedAttrs<'_> {
        SpannedAttrs {
            inner: self.attrs.iter(),
        }
    }
}

/// Parsed from: `#+CAPTION[OPTIONAL]: VALUE`.
///
/// See [`AffiliatedKeywords`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Caption {
    optional: Option<SecondaryString<StandardSet>>,
    value: SecondaryString<StandardSet>,
}

impl Caption {
    pub fn new(value: SecondaryString<StandardSet>) -> Caption {
        Caption {
            optional: None,
            value,
        }
    }
    pub fn with_optional(
        value: SecondaryString<StandardSet>,
        optional: SecondaryString<StandardSet>,
    ) -> Caption {
        Caption {
            optional: Some(optional),
            value,
        }
    }
    pub fn with_option_optional(
        value: SecondaryString<StandardSet>,
        optional: Option<SecondaryString<StandardSet>>,
    ) -> Caption {
        Caption { value, optional }
    }

    pub fn optional(&self) -> &Option<SecondaryString<StandardSet>> {
        &self.optional
    }
    pub fn value(&self) -> &SecondaryString<StandardSet> {
        &self.value
    }
}

/// Parsed from: `#+RESULTS[OPTIONAL]: VALUE`.
///
/// See [`AffiliatedKeywords`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Results {
    optional: Option<String>,
    value: String,
}

/// Parsed from: `#+ATTR_BACKEND: VALUE`.
///
/// See [`AffiliatedKeywords`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr {
    backend: String,
    value: String,
}

/// An iterator over [`Caption`]s.
///
/// This struct is created by the [`captions`][`AffiliatedKeywords::captions`] method
/// on [`AffiliatedKeywords`].
pub struct Captions<'a> {
    inner: slice::Iter<'a, Spanned<Caption>>,
}

impl<'a> Iterator for Captions<'a> {
    type Item = &'a Caption;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|spanned| spanned.value())
    }
}

/// An iterator over [`Caption`]s wrapped in [`SpannedValue`].
///
/// This struct is created by the [`spanned_captions`][`AffiliatedKeywords::spanned_captions`] method
/// on [`AffiliatedKeywords`].
pub struct SpannedCaptions<'a> {
    inner: slice::Iter<'a, Spanned<Caption>>,
}

impl<'a> Iterator for SpannedCaptions<'a> {
    type Item = &'a Spanned<Caption>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over headers.
///
/// This struct is created by the [`headers`][`AffiliatedKeywords::headers`] method
/// on [`AffiliatedKeywords`].
pub struct Headers<'a> {
    inner: slice::Iter<'a, Spanned<String>>,
}

impl<'a> Iterator for Headers<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|spanned| spanned.value())
    }
}

/// An iterator over headers wrapped in [`SpannedValue`].
///
/// This struct is created by the [`spanned_headers`][`AffiliatedKeywords::spanned_headers`] method
/// on [`AffiliatedKeywords`].
pub struct SpannedHeaders<'a> {
    inner: slice::Iter<'a, Spanned<String>>,
}

impl<'a> Iterator for SpannedHeaders<'a> {
    type Item = &'a Spanned<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over [`Attr`]s.
///
/// This struct is created by the [`attrs`][`AffiliatedKeywords::attrs`] method
/// on [`AffiliatedKeywords`].
pub struct Attrs<'a> {
    inner: slice::Iter<'a, Spanned<Attr>>,
}

impl<'a> Iterator for Attrs<'a> {
    type Item = &'a Attr;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|spanned| spanned.value())
    }
}

/// An iterator over [`Attr`]s wrapped in [`SpannedValue`].
///
/// This struct is created by the [`spanned_attrs`][`AffiliatedKeywords::spanned_attrs`] method
/// on [`AffiliatedKeywords`].
pub struct SpannedAttrs<'a> {
    inner: slice::Iter<'a, Spanned<Attr>>,
}

impl<'a> Iterator for SpannedAttrs<'a> {
    type Item = &'a Spanned<Attr>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
