use super::*;
use regex::{self, Regex};

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
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    shared_behavior_data: SharedBehaviorData,
    pub name: String,
    /// True if the entity ended with `{}`.
    pub used_brackets: bool,
}

// test implementation for entity parsing using the combine crate
mod with_combine {
    use super::{Entity, SharedBehaviorData, Span};
    use combine::error::ParseError;
    use combine::parser::char::{char, space};
    use combine::parser::regex::{captures, find};
    use combine::stream::{FullRangeStream, Stream};
    use combine::{choice, many, position, Parser};

    #[test]
    fn test_parse_entity() {
        use combine::stream::state::IndexPositioner;
        use combine::stream::state::State;

        let text = r"\someentity{}  ";
        let expected = Entity {
            shared_behavior_data: SharedBehaviorData {
                span: Span::new(0, 13),
                post_blank: 2,
            },
            name: "someentity".to_string(),
            used_brackets: true,
        };
        let result = parse_entity()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }
    #[test]
    fn test_parse_entity_spaces() {
        use combine::stream::state::IndexPositioner;
        use combine::stream::state::State;

        let text = r"\_  ";
        let expected = Entity {
            shared_behavior_data: SharedBehaviorData {
                span: Span::new(0, 4),
                post_blank: 0,
            },
            name: "_  ".to_string(),
            used_brackets: false,
        };
        let result = parse_entity()
            .easy_parse(State::with_positioner(text, IndexPositioner::new()))
            .map(|t| t.0);
        assert_eq!(result, Ok(expected));
    }

    fn spanned<I, P>(p: P) -> impl Parser<Input = I, Output = (Span, P::Output)>
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

    fn shared_behavior_data<I, P>(
        p: P,
    ) -> impl Parser<Input = I, Output = (SharedBehaviorData, P::Output)>
    where
        I: Stream<Item = char, Position = usize>,
        P: Parser<Input = I>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        (spanned(p), many::<Counter, _>(space())).map(|((span, content), Counter(post_blank))| {
            (SharedBehaviorData { span, post_blank }, content)
        })
    }

    fn parse_entity<'a, I: 'a>() -> impl Parser<Input = I, Output = Entity> + 'a
    where
        I: Stream<Item = char, Range = &'a str, Position = usize> + FullRangeStream,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        use regex::Regex;

        shared_behavior_data(
            char('\\').with(choice((
                find(Regex::new(r"_ +").unwrap()).map(|s: &str| (s.to_string(), false)),
                captures(Regex::new(r"(there4|sup[123]|frac[13][24]|[[:alpha:]]+)(\{\})").unwrap())
                    .map(|vec: Vec<&str>| {
                        let name = vec[1];
                        let brackets = vec[2];
                        (name.to_string(), brackets == "{}")
                    }),
            ))),
        )
        .map(|(shared_behavior_data, (name, used_brackets))| Entity {
            shared_behavior_data,
            name,
            used_brackets,
        })
    }
}

impl Parse for Entity {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?mx)\A\\(?:
                (?P<spaces>_\ +)
                |
                (?P<name>there4|sup[123]|frac[13][24]|[[:alpha:]]+)
                (?P<post>$|\{}|[^[:alpha:]])
                )"
            )
            .unwrap();
        }

        fn collect_data(
            context: &mut Context,
            captures: &regex::Captures<'_>,
        ) -> Result<(String, bool), !> {
            let name_group = captures
                .name("spaces")
                .or_else(|| captures.name("name"))
                .unwrap();
            let name = name_group.as_str().to_string();
            let post_group = captures.name("post");
            let post = post_group.map(|m| m.as_str());

            // skip over name
            context.move_cursor_forward(name_group.end());

            let used_brackets = post == Some("{}");
            if used_brackets {
                // skip over brackets
                context.move_cursor_forward(2);
            }

            Ok((name, used_brackets))
        }

        fn from_collected_data(
            (name, used_brackets): (String, bool),
            shared_behavior_data: SharedBehaviorData,
        ) -> Result<Entity, !> {
            Ok(Entity {
                shared_behavior_data,
                name,
                used_brackets,
            })
        }

        parser.parse_object(&RE, collect_data, from_collected_data)
    }
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
    use crate::parsing::Input;

    #[test]
    fn test_parse_spaces_entity() {
        let s = r"\_ ";
        let mut parser = Input::new(s).into();
        let parsed = Entity::parse(&mut parser).unwrap();

        assert_eq!(
            parsed,
            Entity {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(0, 2),
                    post_blank: 0,
                },
                name: "_ ".to_string(),
                used_brackets: false,
            }
        );
        assert_eq!(parser.cursor_pos(), 3);
        assert_eq!(parsed.to_string(), s);
    }

    #[test]
    fn test_parse_entity() {
        let s = r"\name";
        let mut parser = Input::new(s).into();
        let parsed = Entity::parse(&mut parser).unwrap();

        assert_eq!(
            parsed,
            Entity {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(0, 4),
                    post_blank: 0,
                },
                name: "name".to_string(),
                used_brackets: false,
            }
        );
        assert_eq!(parser.cursor_pos(), 5);
        assert_eq!(parsed.to_string(), s);
    }

    #[test]
    fn test_parse_entity_with_brackets() {
        let s = r"\name{}";
        let mut parser = Input::new(s).into();
        let parsed = Entity::parse(&mut parser).unwrap();

        assert_eq!(
            parsed,
            Entity {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(0, 6),
                    post_blank: 0,
                },
                name: "name".to_string(),
                used_brackets: true,
            }
        );
        assert_eq!(parser.cursor_pos(), 7);
        assert_eq!(parsed.to_string(), s);
    }

    #[test]
    fn test_parse_entity_with_brackets_and_post_blanks() {
        let s = "\\name{}\t\t\t \t";
        let mut parser = Input::new(s).into();
        let parsed = Entity::parse(&mut parser).unwrap();

        assert_eq!(
            parsed,
            Entity {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(0, 6),
                    post_blank: 5,
                },
                name: "name".to_string(),
                used_brackets: true,
            }
        );
        assert_eq!(parser.cursor_pos(), 12);
        assert_eq!(parsed.to_string(), r"\name{}");
    }

    #[test]
    fn test_parse_entity_with_non_alpha_post() {
        let s = r"\name6";
        let mut parser = Input::new(s).into();
        let parsed = Entity::parse(&mut parser).unwrap();

        assert_eq!(
            parsed,
            Entity {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(0, 4),
                    post_blank: 0,
                },
                name: "name".to_string(),
                used_brackets: false,
            }
        );
        assert_eq!(parser.cursor_pos(), 5);
        assert_eq!(parsed.to_string(), r"\name");
    }
}
