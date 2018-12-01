//! `AffiliatedKeywords` holds the attributes affiliated with an element.

use super::*;
use std::fmt;
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

impl fmt::Display for AffiliatedKeywords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter().format("\n"))
    }
}

impl<A> std::iter::FromIterator<A> for AffiliatedKeywords
where
    A: Into<AffiliatedKeyword>,
{
    /// `AffiliatedKeywords` can only hold one of [`AffiliatedKeyword::Name`],
    /// [`AffiliatedKeyword::Plot`] and [`AffiliatedKeyword::Results`]. If there are multiple in
    /// the iterator only the last of each (if any) will be kept.
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
    /// `AffiliatedKeywords` can only hold one of [`AffiliatedKeyword::Name`],
    /// [`AffiliatedKeyword::Plot`] and [`AffiliatedKeyword::Results`]. If there are multiple in
    /// the iterator only the last of each (if any) will be kept.
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>,
    {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl IntoIterator for AffiliatedKeywords {
    type Item = AffiliatedKeyword;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self
                .captions
                .into_iter()
                .map(AffiliatedKeyword::Caption as fn(Spanned<Caption>) -> AffiliatedKeyword)
                .chain(
                    self.headers
                        .into_iter()
                        .map(AffiliatedKeyword::Header as fn(Spanned<String>) -> AffiliatedKeyword),
                )
                .chain(
                    self.name
                        .into_iter()
                        .map(AffiliatedKeyword::Name as fn(Spanned<String>) -> AffiliatedKeyword),
                )
                .chain(
                    self.plot
                        .into_iter()
                        .map(AffiliatedKeyword::Plot as fn(Spanned<String>) -> AffiliatedKeyword),
                )
                .chain(
                    self.results.into_iter().map(
                        AffiliatedKeyword::Results as fn(Spanned<Results>) -> AffiliatedKeyword,
                    ),
                )
                .chain(
                    self.attrs
                        .into_iter()
                        .map(AffiliatedKeyword::Attr as fn(Spanned<Attr>) -> AffiliatedKeyword),
                ),
        }
    }
}

impl AffiliatedKeywords {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.captions.is_empty()
            && self.headers.is_empty()
            && self.name.is_none()
            && self.plot.is_none()
            && self.results.is_none()
            && self.attrs.is_empty()
    }

    /// Adds a single [`AffiliatedKeyword`] to the `AffiliatedKeywords` struct.
    ///
    /// `AffiliatedKeywords` can only hold one of [`AffiliatedKeyword::Name`],
    /// [`AffiliatedKeyword::Plot`] and [`AffiliatedKeyword::Results`]. These will overwrite the
    /// values that are already present (if any). The replaces value will be returned. If there are
    /// multiple occurenses of `Name`, `Plot` or `Results` in the iterator only the last of each
    /// (if any) will be kept.
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

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = AffiliatedKeyword> + 'a {
        self.captions
            .iter()
            .map(AffiliatedKeyword::from_borrowed_caption)
            .chain(
                self.headers
                    .iter()
                    .map(AffiliatedKeyword::from_borrowed_header),
            )
            .chain(self.name.iter().map(AffiliatedKeyword::from_borrowed_name))
            .chain(self.plot.iter().map(AffiliatedKeyword::from_borrowed_plot))
            .chain(
                self.results
                    .iter()
                    .map(AffiliatedKeyword::from_borrowed_results),
            )
            .chain(self.attrs.iter().map(AffiliatedKeyword::from_borrowed_attr))
    }
}

/// Represents a single affiliated keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AffiliatedKeyword {
    Caption(Spanned<Caption>),
    Header(Spanned<String>),
    Name(Spanned<String>),
    Plot(Spanned<String>),
    Results(Spanned<Results>),
    Attr(Spanned<Attr>),
}

impl fmt::Display for AffiliatedKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AffiliatedKeyword::Caption(caption) => write!(f, "{}", caption.value()),
            AffiliatedKeyword::Header(header) => write!(f, "#+HEADER: {}", header.value()),
            AffiliatedKeyword::Name(name) => write!(f, "#+NAME: {}", name.value()),
            AffiliatedKeyword::Plot(plot) => write!(f, "#+PLOT: {}", plot.value()),
            AffiliatedKeyword::Results(results) => write!(f, "{}", results.value()),
            AffiliatedKeyword::Attr(attr) => write!(f, "{}", attr.value()),
        }
    }
}

impl AffiliatedKeyword {
    fn from_borrowed_caption(caption: &Spanned<Caption>) -> AffiliatedKeyword {
        AffiliatedKeyword::Caption(caption.clone())
    }
    fn from_borrowed_header(header: &Spanned<String>) -> AffiliatedKeyword {
        AffiliatedKeyword::Header(header.clone())
    }
    fn from_borrowed_name(name: &Spanned<String>) -> AffiliatedKeyword {
        AffiliatedKeyword::Name(name.clone())
    }
    fn from_borrowed_plot(plot: &Spanned<String>) -> AffiliatedKeyword {
        AffiliatedKeyword::Plot(plot.clone())
    }
    fn from_borrowed_results(results: &Spanned<Results>) -> AffiliatedKeyword {
        AffiliatedKeyword::Results(results.clone())
    }
    fn from_borrowed_attr(attr: &Spanned<Attr>) -> AffiliatedKeyword {
        AffiliatedKeyword::Attr(attr.clone())
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

impl fmt::Display for Caption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.optional {
            Some(optional) => write!(f, "#+CAPTION[{}]: {}", optional, self.value),
            None => write!(f, "#+CAPTION: {}", self.value),
        }
    }
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
    value: String,
    optional: Option<String>,
}

impl Results {
    pub fn new(value: String, optional: Option<String>) -> Self {
        Results { value, optional }
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.optional {
            Some(optional) => write!(f, "#+RESULTS[{}]: {}", optional, self.value),
            None => write!(f, "#+RESULTS: {}", self.value),
        }
    }
}

/// Parsed from: `#+ATTR_BACKEND: VALUE`.
///
/// See [`AffiliatedKeywords`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr {
    backend: String,
    value: String,
}

impl Attr {
    pub fn new(backend: String, value: String) -> Self {
        Attr { backend, value }
    }
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#+ATTR_{}: {}", self.backend, self.value)
    }
}

/// Iterators for the different fields of `AffiliatedKeywords`.
mod iter {
    use super::*;
    use std::iter::{Chain, Map};
    use std::option;
    use std::vec;

    // XXX: This type is humongous. But this is easier than implementing some sort of state to know
    // what field we are currently in and where in that field if it is a vector and where to go to next
    // and what happens if one of the fields is empty.
    pub struct IntoIter {
        pub(super) inner: Chain<
            Chain<
                Chain<
                    Chain<
                        Chain<
                            Map<
                                vec::IntoIter<Spanned<Caption>>,
                                fn(Spanned<Caption>) -> AffiliatedKeyword,
                            >,
                            Map<
                                vec::IntoIter<Spanned<String>>,
                                fn(Spanned<String>) -> AffiliatedKeyword,
                            >,
                        >,
                        Map<
                            option::IntoIter<Spanned<String>>,
                            fn(Spanned<String>) -> AffiliatedKeyword,
                        >,
                    >,
                    Map<
                        option::IntoIter<Spanned<String>>,
                        fn(Spanned<String>) -> AffiliatedKeyword,
                    >,
                >,
                Map<option::IntoIter<Spanned<Results>>, fn(Spanned<Results>) -> AffiliatedKeyword>,
            >,
            Map<vec::IntoIter<Spanned<Attr>>, fn(Spanned<Attr>) -> AffiliatedKeyword>,
        >,
    }

    impl Iterator for IntoIter {
        type Item = AffiliatedKeyword;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

    /// An iterator over [`Caption`]s.
    ///
    /// This struct is created by the [`captions`][`AffiliatedKeywords::captions`] method
    /// on [`AffiliatedKeywords`].
    pub struct Captions<'a> {
        pub(super) inner: slice::Iter<'a, Spanned<Caption>>,
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
        pub(super) inner: slice::Iter<'a, Spanned<Caption>>,
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
        pub(super) inner: slice::Iter<'a, Spanned<String>>,
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
        pub(super) inner: slice::Iter<'a, Spanned<String>>,
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
        pub(super) inner: slice::Iter<'a, Spanned<Attr>>,
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
        pub(super) inner: slice::Iter<'a, Spanned<Attr>>,
    }

    impl<'a> Iterator for SpannedAttrs<'a> {
        type Item = &'a Spanned<Attr>;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }
}
pub use self::iter::{
    Attrs, Captions, Headers, IntoIter, SpannedAttrs, SpannedCaptions, SpannedHeaders,
};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashSet;

    fn span() -> impl Strategy<Value = Span> {
        any::<usize>()
            .prop_flat_map(|start| (Just(start), start..))
            .prop_map(|(start, end)| Span::new(start, end))
    }

    prop_compose! {
        fn caption()(
            span in span(),
            optional in "[a-zA-Z_]{1,}".prop_perturb(|o, mut rng| if rng.gen() { Some(o) } else { None }),
            value in "[a-zA-Z_]{1,}"
        ) -> Spanned<Caption> {
            let value = SecondaryString::with_one(StandardSet::RawString(value));
            let optional = optional.map(|value| SecondaryString::with_one(StandardSet::RawString(value)));
            let caption = Caption::with_option_optional(value, optional);
            Spanned::with_span(caption, span)
        }
    }
    prop_compose! {
        fn header()(
            span in span(),
            value in "[a-zA-Z_]{1,}"
        ) -> Spanned<String> {
            Spanned::with_span(value, span)
        }
    }
    prop_compose! {
        fn name()(
            span in span(),
            value in "[a-zA-Z_]{1,}"
        ) -> Spanned<String> {
            Spanned::with_span(value, span)
        }
    }
    prop_compose! {
        fn plot()(
            span in span(),
            value in "[a-zA-Z_]{1,}"
        ) -> Spanned<String> {
            Spanned::with_span(value, span)
        }
    }
    prop_compose! {
        fn results()(
            span in span(),
            optional in "[a-zA-Z_]{1,}".prop_perturb(|o, mut rng| if rng.gen() { Some(o) } else { None }),
            value in "[a-zA-Z_]{1,}"
        ) -> Spanned<Results> {
            let caption = Results { value, optional, };
            Spanned::with_span(caption, span)
        }
    }
    prop_compose! {
        fn attr()(
            span in span(),
            backend in "[a-zA-Z_]{1,}",
            value in "[a-zA-Z_]{1,}"
        ) -> Spanned<Attr> {
            let attr = Attr { backend, value, };
            Spanned::with_span(attr, span)
        }
    }

    proptest! {
        // TODO un-ignore when parsing is implemented
        #[test]
        #[ignore]
        fn test_affiliated_keywords_into_iter(
            captions in prop::collection::vec(caption(), 0..10),
            headers in prop::collection::vec(header(), 0..10),
            name in prop::option::of(name()),
            plot in prop::option::of(plot()),
            results in prop::option::of(results()),
            attrs in prop::collection::vec(attr(), 0..10),
        ) {
            let expected: HashSet<_> = captions.clone().into_iter().map(AffiliatedKeyword::Caption)
                .chain(headers.clone().into_iter().map(AffiliatedKeyword::Header))
                .chain(name.clone().into_iter().map(AffiliatedKeyword::Name))
                .chain(plot.clone().into_iter().map(AffiliatedKeyword::Plot))
                .chain(results.clone().into_iter().map(AffiliatedKeyword::Results))
                .chain(attrs.clone().into_iter().map(AffiliatedKeyword::Attr))
                .collect();
            let aks = AffiliatedKeywords {
                captions,
                headers,
                name,
                plot,
                results,
                attrs,
            };
            let actual: HashSet<_> = aks.into_iter().collect();
            assert_eq!(expected, actual);
        }
    }

    proptest! {
        // TODO un-ignore when parsing is implemented
        #[test]
        #[ignore]
        fn test_parse_affiliated_keywords(
            captions in prop::collection::vec(caption(), 0..2),
            headers in prop::collection::vec(header(), 0..2),
            name in prop::option::of(name()),
            plot in prop::option::of(plot()),
            results in prop::option::of(results()),
            attrs in prop::collection::vec(attr(), 0..4),
        ) {
            let expected = AffiliatedKeywords {
                captions,
                headers,
                name,
                plot,
                results,
                attrs,
            };
            prop_assume!(!expected.is_empty());
            let text = expected.to_string();
            println!("{}", text);
            let result: AffiliatedKeywords = unimplemented!(); // parse text

            assert_eq!(text, result.to_string());
            assert_eq!(expected.captions().collect::<HashSet<_>>(), result.captions().collect());
            assert_eq!(expected.headers().collect::<HashSet<_>>(), result.headers().collect());
            assert_eq!(expected.name(), result.name());
            assert_eq!(expected.plot(), result.plot());
            assert_eq!(expected.results(), result.results());
            assert_eq!(expected.attrs().collect::<HashSet<_>>(), result.attrs().collect());
        }
    }

    // TODO un-ignore when parsing is implemented
    #[test]
    #[ignore]
    fn test_parse_affiliated_keywords_attr() {
        let text = "#+ATTR_something: value";
        let result = unimplemented!();
        let mut expected = AffiliatedKeywords::new();
        expected.push(AffiliatedKeyword::Attr(Spanned::with_span(
            Attr {
                backend: String::from("something"),
                value: String::from("value"),
            },
            Span::new(0, 23),
        )));

        assert_eq!(expected, result);
        assert_eq!(text, result.to_string());
    }
}

