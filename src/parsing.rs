//! Contains the types and traits needed for parsing.
use combine::parser::char::space;
use combine::stream::state::{IndexPositioner, State};
use combine::stream::{easy, FullRangeStream, RangeStreamOnce};
use combine::{many, position, value, ParseError, Parser, Stream};
use crate::types::affiliated_keywords::{self, AffiliatedKeyword, AffiliatedKeywords};
use crate::types::*;
use regex::{Captures, Match, Regex};

pub fn spanned<I, P>(p: P) -> impl Parser<Input = I, Output = (Span, P::Output)>
where
    I: Stream<Position = usize>,
    P: Parser<Input = I>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (position(), p, position()).map(|(start, content, end)| (Span::new(start, end), content))
}

#[derive(Debug, Default)]
struct Counter(usize);

impl<A> Extend<A> for Counter {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>,
    {
        self.0 += iter.into_iter().count();
    }
}

pub fn shared_behavior_data<I, P>(
    p: P,
) -> impl Parser<Input = I, Output = (SharedBehaviorData, P::Output)>
where
    I: Stream<Item = char, Position = usize>,
    P: Parser<Input = I>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (spanned(p), many::<Counter, _>(space())).map(|((span, content), Counter(post_blank))| {
        (SharedBehaviorData::new(span, post_blank), content)
    })
}

pub fn object<I, P>(p: P) -> impl Parser<Input = I, Output = (SharedBehaviorData, P::Output)>
where
    I: Stream<Item = char, Position = usize>,
    P: Parser<Input = I>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    shared_behavior_data(p)
}

fn _content_data<'a, I, T>() -> impl Parser<Input = I, Output = ContentData<T>> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize> + FullRangeStream + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    T: Clone + 'a,
{
    // TODO replace value(()) with actual content parsing
    (position(), value(()), position()).map(|(start, _content, end)| {
        let span = Span::new(start, end);
        //ContentData::new(span, content)
        ContentData::empty(span)
    })
}

pub fn content_data<'a, T: Clone + 'a>(
    input: &'a str,
    position: usize,
) -> Result<
    (ContentData<T>, State<&'a str, IndexPositioner>),
    easy::ParseError<State<&'a str, IndexPositioner>>,
> {
    let input = State::with_positioner(input, IndexPositioner::new_with_position(position));
    _content_data().easy_parse(input)
}

use combine::parser::char::string;
use combine::parser::item::token;
use combine::parser::range::recognize;
use combine::parser::repeat::{sep_by, skip_until};
use combine::{between, choice, one_of, optional, skip_many};

pub fn affiliated_keywords<'a, I: 'a>() -> impl Parser<Input = I, Output = AffiliatedKeywords> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize> + RangeStreamOnce,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    sep_by(affiliated_keyword(), token('\n')).flat_map(|aks: Vec<_>| {
        // validate the affiliated keywords that can't accur more than once
        Ok(aks.into_iter().collect())
    })
}

pub fn affiliated_keyword<'a, I: 'a>() -> impl Parser<Input = I, Output = AffiliatedKeyword> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize> + RangeStreamOnce,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        caption().map(AffiliatedKeyword::Caption),
        header().map(AffiliatedKeyword::Header),
        //name().map(AffiliatedKeyword::Name),
        //plot().map(AffiliatedKeyword::Plot),
        //results().map(AffiliatedKeyword::Results),
        //attrs().map(AffiliatedKeyword::Attr),
    ))
}

pub fn caption<'a, I: 'a>(
) -> impl Parser<Input = I, Output = Spanned<affiliated_keywords::Caption>> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize> + RangeStreamOnce,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        position(),
        string("#+CAPTION"),
        optional(between(token('['), token(']'), secondary_string())),
        string(": "),
        secondary_string(),
        position(),
    )
        .map(|(start, _, optional, _, value, end)| {
            let span = Span::new(start, end);
            let caption = affiliated_keywords::Caption::with_option_optional(value, optional);
            Spanned::new(span, caption)
        })
}
pub fn header<'a, I: 'a>() -> impl Parser<Input = I, Output = Spanned<String>> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize> + RangeStreamOnce,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        position(),
        choice((string("#+HEADER: "), string("#+HEADERS: "))),
        recognize(skip_until(space())),
        skip_many(one_of(" \t".chars())),
        position(),
    )
        .map(|(start, _, value, _, end): (_, _, &str, _, _)| {
            let span = Span::new(start, end);
            let header = value.to_string();
            Spanned::new(span, header)
        })
}

pub fn secondary_string<'a, I: 'a, T>() -> impl Parser<Input = I, Output = SecondaryString<T>> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize> + RangeStreamOnce,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
    T: AsRawString + Clone + 'a,
{
    // TODO same trick as with content data
    value(SecondaryString::new())
}

#[derive(Debug, Clone)]
pub struct Context {}
