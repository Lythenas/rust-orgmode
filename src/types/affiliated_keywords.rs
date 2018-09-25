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
    captions: Vec<SpannedValue<Caption>>,
    headers: Vec<SpannedValue<String>>,
    name: Option<SpannedValue<String>>,
    plot: Option<SpannedValue<String>>,
    results: Option<SpannedValue<Results>>,
    attrs: Vec<SpannedValue<Attr>>,
}

impl AffiliatedKeywords {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn captions(&self) -> Captions {
        Captions {
            inner: self.captions.iter(),
        }
    }
    pub fn spanned_captions(&self) -> SpannedCaptions {
        SpannedCaptions {
            inner: self.captions.iter(),
        }
    }
    pub fn headers(&self) -> Headers {
        Headers {
            inner: self.headers.iter(),
        }
    }
    pub fn spanned_headers(&self) -> SpannedHeaders {
        SpannedHeaders {
            inner: self.headers.iter(),
        }
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref().map(|spanned| spanned.value())
    }
    pub fn spanned_name(&self) -> Option<&SpannedValue<String>> {
        self.name.as_ref()
    }
    pub fn plot(&self) -> Option<&String> {
        self.plot.as_ref().map(|spanned| spanned.value())
    }
    pub fn spanned_plot(&self) -> Option<&SpannedValue<String>> {
        self.plot.as_ref()
    }
    pub fn results(&self) -> Option<&Results> {
        self.results.as_ref().map(|spanned| spanned.value())
    }
    pub fn spanned_results(&self) -> Option<&SpannedValue<Results>> {
        self.results.as_ref()
    }
    pub fn attrs(&self) -> Attrs {
        Attrs {
            inner: self.attrs.iter(),
        }
    }
    pub fn spanned_attrs(&self) -> SpannedAttrs {
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
    inner: slice::Iter<'a, SpannedValue<Caption>>,
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
    inner: slice::Iter<'a, SpannedValue<Caption>>,
}

impl<'a> Iterator for SpannedCaptions<'a> {
    type Item = &'a SpannedValue<Caption>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over headers.
///
/// This struct is created by the [`headers`][`AffiliatedKeywords::headers`] method
/// on [`AffiliatedKeywords`].
pub struct Headers<'a> {
    inner: slice::Iter<'a, SpannedValue<String>>,
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
    inner: slice::Iter<'a, SpannedValue<String>>,
}

impl<'a> Iterator for SpannedHeaders<'a> {
    type Item = &'a SpannedValue<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over [`Attr`]s.
///
/// This struct is created by the [`attrs`][`AffiliatedKeywords::attrs`] method
/// on [`AffiliatedKeywords`].
pub struct Attrs<'a> {
    inner: slice::Iter<'a, SpannedValue<Attr>>,
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
    inner: slice::Iter<'a, SpannedValue<Attr>>,
}

impl<'a> Iterator for SpannedAttrs<'a> {
    type Item = &'a SpannedValue<Attr>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
