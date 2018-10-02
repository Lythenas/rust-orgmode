//! Contains the types and traits needed for parsing.
use combine::parser::char::space;
use combine::stream::FullRangeStream;
use combine::{many, position, value, ParseError, Parser, Stream};
use crate::types::affiliated_keywords::AffiliatedKeywords;
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

pub fn content_data<'a, I, T>() -> impl Parser<Input = I, Output = ContentData<T>> + 'a
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

#[derive(Debug, Clone)]
pub struct Context {}
