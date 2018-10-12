use super::*;
use combine::parser::char::string;
use combine::parser::range::recognize;
use combine::parser::regex::{captures, find};
use combine::parser::repeat::skip_until;
use combine::stream::state::{IndexPositioner, State};
use combine::stream::{easy, FullRangeStream, Stream, StreamOnce};
use combine::{one_of, optional, position, skip_many, token, value, ParseError, Parser};
use crate::parsing::{content_data, spanned};
use regex::Regex;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Drawer {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<Vec<ElementSet>>,
    pub name: String,
    // hiddenp: bool,
}

impl Parent<Vec<ElementSet>> for Drawer {
    fn content(&self) -> Option<&Spanned<Vec<ElementSet>>> {
        Some(&self.content)
    }
}

impl Element for Drawer {}
impl GreaterElement for Drawer {}
impl HasAffiliatedKeywords for Drawer {
    fn affiliated_keywords(&self) -> Option<&Spanned<AffiliatedKeywords>> {
        self.affiliated_keywords.as_ref()
    }
}

fn parse_drawer<'a, I: 'a>() -> impl Parser<Input = I, Output = Spanned<Drawer>> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize>
        + FullRangeStream
        + StreamOnce<Error = combine::easy::Errors<char, &'a str, usize>>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    lazy_static! {
        static ref RE_START: Regex = Regex::new(r"^[ \t]*:(\S+):[ \t]*\n").unwrap();
        static ref RE_END: Regex = Regex::new(r"^[ \t]*:END:[ \t]*\n?").unwrap();
    }

    spanned(
        captures(&*RE_START)
            .map(|vec: Vec<&str>| vec[1])
            .then(|name: &str| {
                (
                    value(name.to_string()),
                    position(),
                    recognize(skip_until(find(&*RE_END))),
                )
                    .flat_map(|(name, position, content_str)| {
                        content_data(content_str, position)
                            .map(|(content_data, _rest)| (name, content_data))
                        // TODO figure out what to do when there is still a res
                    })
                    .skip(find(&*RE_END))
            }),
    )
    .map(|spanned| {
        spanned.map_value(|(name, content)| Drawer {
            affiliated_keywords: None,
            content,
            name,
        })
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
    use crate::types::IntoSpanned;

    #[test]
    fn empty_drawer() {
        let text = ":something:\n:END:";
        let expected = Drawer {
            affiliated_keywords: None,
            content: Spanned::new(Span::new(12, 12), Vec::new()),
            name: "something".to_string(),
        }
        .into_spanned(Span::new(0, 17));
        let result = parse_drawer()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn fails_when_there_is_something_in_front_of_the_drawer() {
        let text = "fail:something:\n:END:";
        let result = parse_drawer()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert!(result.is_err());
    }

    // TODO add more tests as soon as node property parsing is implemented
}
