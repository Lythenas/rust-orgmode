use super::*;
use combine::error::ParseError;
use combine::parser::char::string;
use combine::parser::range::{range, recognize};
use combine::parser::regex::captures;
use combine::parser::repeat::skip_until;
use combine::stream::{FullRangeStream, Stream, StreamOnce};
use combine::{one_of, optional, position, skip_many, value, Parser};
use crate::parsing::{content_data, spanned};
use regex::Regex;

/// A special block.
///
/// # Semantics
///
/// Any block with name that is not recognized as another block is a special block.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_NAME
/// CONTENTS
/// #+END_NAME
/// ```
///
/// `NAME` can contain any non-whitespace character.
///
/// `CONTENTS` can contain anything except a line `#+END_NAME` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// TODO not sure if this is actually a greater element
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecialBlock {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    content: Spanned<String>,
    pub name: String,
    // hiddenp: bool
}
impl Parent<String> for SpecialBlock {
    fn content(&self) -> Option<&Spanned<String>> {
        Some(&self.content)
    }
}
impl HasAffiliatedKeywords for SpecialBlock {
    fn affiliated_keywords(&self) -> Option<&Spanned<AffiliatedKeywords>> {
        self.affiliated_keywords.as_ref()
    }
}
impl Element for SpecialBlock {}
impl GreaterElement for SpecialBlock {}

// use combine::stream::RangeStreamOnce;
// fn owned_string<'a, I: 'a>(s: String) -> impl Parser<Input = I, Output = &'a str>
// where
//     I: Stream<Item = char, Range = &'a str> + RangeStreamOnce,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
// {
//     use combine::tokens2;
//     recognize(
//         tokens2(|l,r| l == r, s.chars().collect::<Vec<_>>().into_iter())
//     )
// }

fn parse_special_block<'a, I: 'a>() -> impl Parser<Input = I, Output = Spanned<SpecialBlock>> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize>
        + FullRangeStream
        + StreamOnce<Error = combine::easy::Errors<char, &'a str, usize>>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    lazy_static! {
        static ref RE_START: Regex = Regex::new(r"^([ \t]*)#\+BEGIN_(\S+)[ \t]*\n").unwrap();
    }

    spanned(
        captures(&*RE_START)
            .map(|vec: Vec<&str>| vec[2])
            .then(|name| {
                let find_end = || {
                    (
                        skip_many(one_of(" \t".chars())),
                        string("#+END_"),
                        range(name),
                        skip_many(one_of(" \t".chars())),
                        optional(string("\n")),
                    )
                };
                (
                    value(name.to_string()),
                    position(),
                    recognize(skip_until(find_end())),
                )
                    .flat_map(|(name, position, content_str): (String, usize, &str)| {
                        content_data(content_str, position)
                            .map(|(content_data, _rest)| (name, content_data))
                    })
                    .skip(find_end())
            }),
    )
    .map(|spanned| {
        spanned.map_value(|(name, content)| SpecialBlock {
            affiliated_keywords: None,
            content,
            name,
        })
    })
}

impl fmt::Display for SpecialBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#+BEGIN_{}", self.name)?;
        for line in self.content() {
            writeln!(f, "{}", line)?;
        }
        write!(f, "#+END_{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::stream::state::{IndexPositioner, State};
    use crate::types::IntoSpanned;

    #[test]
    fn empty_block() {
        let text = "#+BEGIN_something\n#+END_something";
        let expected = SpecialBlock {
            affiliated_keywords: None,
            content: Spanned::new(Span::new(18, 18), String::new()),
            name: "something".to_string(),
        }
        .into_spanned(Span::new(0, 33));
        let result = parse_special_block()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn fails_when_there_is_something_in_front_of_the_block() {
        let text = "fail#+BEGIN_something\n#+END_something";
        let result = parse_special_block()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert!(result.is_err());
    }

    // TODO add more tests once content parsing is implemented
}
