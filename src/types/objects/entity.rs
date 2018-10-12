use super::*;
use regex::Regex;

/// An entity.
///
/// # Semantics
///
/// An entity is a special character which has to be exported differently to different formats.
///
/// # Syntax
///
/// ```text
/// \NAME POST
/// ```
///
/// `NAME` has to have a valid association in [`entities`] or in the used defined variable
/// `org_entities_user` which can be configured before parsing. It has to conform to the
/// following regular expression: `(_ +)|(there4|frac[13][24]|[a-zA-Z]+)` (this restriction
/// could be removed in the future).
///
/// `POST` is the end of line, the string `{}` or a non-alphabetical character (e.g. a
/// whitespace). It isn't separated from `NAME` by any whitespace.
///
/// [`entities`]: ../../entities/index.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    pub name: String,
    /// True if the entity ended with `{}`.
    pub used_brackets: bool,
}

use combine::error::ParseError;
use combine::parser::char::char;
use combine::parser::regex::{captures, find};
use combine::stream::{FullRangeStream, Stream};
use combine::{choice, Parser};
use crate::parsing::object;

fn parse_entity<'a, I: 'a>() -> impl Parser<Input = I, Output = Spanned<Entity>> + 'a
where
    I: Stream<Item = char, Range = &'a str, Position = usize> + FullRangeStream,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    lazy_static! {
        static ref RE_SPACES: Regex = Regex::new(r"_ +").unwrap();
        static ref RE_OTHER: Regex =
            Regex::new(r"^(there4|sup[123]|frac[13][24]|[[:alpha:]]+)((?:\{\})?)").unwrap();
    };
    object(char('\\').with(choice((
        find(&*RE_SPACES).map(|s: &str| (s.to_string(), false)),
        captures(&*RE_OTHER).map(|vec: Vec<&str>| {
            let name = vec[1];
            let brackets = vec[2];
            (name.to_string(), brackets == "{}")
        }),
    ))))
    .map(|spanned| {
        spanned.map_value(|(name, used_brackets)| Entity {
            name,
            used_brackets,
        })
    })
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let brackets = if self.used_brackets { "{}" } else { "" };
        write!(f, r"\{}{}", self.name, brackets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::stream::state::IndexPositioner;
    use combine::stream::state::State;
    use crate::types::IntoSpanned;

    #[test]
    fn with_brackets() {
        let text = r"\someentity{}";
        let expected = Entity {
            name: "someentity".to_string(),
            used_brackets: true,
        }
        .into_spanned(Span::new(0, 13));
        let result = parse_entity()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn with_post_blank() {
        let text = r"\someentity  ";
        let expected = Entity {
            name: "someentity".to_string(),
            used_brackets: false,
        }
        .into_spanned(Span::new(0, 11));
        let result = parse_entity()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn with_brackets_and_post_blank() {
        let text = r"\someentity{}  ";
        let expected = Entity {
            name: "someentity".to_string(),
            used_brackets: true,
        }
        .into_spanned(Span::new(0, 13));
        let result = parse_entity()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn spaces_entity() {
        let text = r"\_  ";
        let expected = Entity {
            name: "_  ".to_string(),
            used_brackets: false,
        }
        .into_spanned(Span::new(0, 4));
        let result = parse_entity()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn fails_when_there_is_something_in_front() {
        let text = r"fail\someentity{}";
        let result = parse_entity()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert!(result.is_err());
    }

}
