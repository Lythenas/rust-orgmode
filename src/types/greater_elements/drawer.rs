use super::*;
use combine::parser::char::string;
use combine::parser::range::recognize;
use combine::parser::repeat::skip_until;
use combine::stream::state::{IndexPositioner, State};
use combine::stream::{easy, FullRangeStream, Stream, StreamOnce};
use combine::{one_of, optional, position, skip_many, token, value, ParseError, Parser};
use crate::parsing::{content_data, shared_behavior_data};
use std::fmt;

/// A drawer to hide content.
///
/// # Semantics
///
/// Used to hide content in the editor and when exporting. Drawers can usually be opened and
/// closed in the editor.
///
/// # Syntax
///
/// ```text
/// :NAME:
/// CONTENTS
/// :END:
/// ```
///
/// `NAME` can contain any word-constituent characters, hyphens and underscores.
///
/// `CONTENTS` can contain any element except a [`Headline`] and another drawer.
///
/// Drawers can be indented.
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Drawer {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: Spanned<AffiliatedKeywords>,
    content_data: ContentData<ElementSet>,
    pub name: String,
    // hiddenp: bool,
}

fn parse_drawer<'a, I: 'a>() -> impl Parser<Input = I, Output = Drawer> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize>
        + FullRangeStream
        + StreamOnce<Error = combine::easy::Errors<char, &'a str, usize>>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let first_line = (skip_many(one_of(" \t".chars())), token(':'))
        .with(recognize(skip_until(token(':'))))
        .skip((token(':'), skip_many(one_of(" \t".chars())), string("\n")));
    let end_line = || {
        (
            skip_many(one_of(" \t".chars())),
            string(":END:"),
            skip_many(one_of(" \t".chars())),
            optional(string("\n")),
        )
    };

    shared_behavior_data(first_line.then(move |name: &str| {
        (
            value(name.to_string()),
            position(),
            recognize(skip_until(end_line())),
        )
            .flat_map(|(name, position, content_str)| {
                content_data(content_str, position)
                    .map(|(content_data, _rest)| (name, content_data))
                // TODO figure out what to do when there is still a res
            })
            .skip(end_line())
    }))
    .map(|(shared_behavior_data, (name, content_data))| Drawer {
        shared_behavior_data,
        affiliated_keywords_data: Spanned::new(Span::new(0, 0), AffiliatedKeywords::default()),
        content_data,
        name,
    })
}

impl fmt::Display for Drawer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ":{}:", self.name)?;
        for _line in self.content() {
            // TODO this should work once all elements impl Display (also impl Display for ElementSet)
            //writeln!(f, "{}", line)?;
        }
        write!(f, ":END:")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::stream::state::{IndexPositioner, State};

    #[test]
    fn test_drawer_empty() {
        let text = ":something:\n:END:";
        let expected = Drawer {
            shared_behavior_data: SharedBehaviorData {
                span: Span::new(0, 17),
            },
            affiliated_keywords_data: Spanned::new(Span::new(0, 0), AffiliatedKeywords::default()),
            content_data: ContentData::empty(Span::new(12, 12)),
            name: "something".to_string(),
        };
        let result = parse_drawer()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    // TODO add more tests as soon as node property parsing is implemented
}
