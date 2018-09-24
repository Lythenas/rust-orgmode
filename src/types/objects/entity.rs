use super::*;

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

impl Parse for Entity {
    fn parse(input: &mut Input) -> Result<Self, ParseError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?mx)\A\\(?:
                (?P<spaces>_\ +)
                |
                (?P<name>there4|sup[123]|frac[13][24]|[[:alpha:]]+)
                (?P<post>$|\{}|[^[:alpha:]])
                )"
            ).unwrap();
        }

        let start = input.cursor;

        if let Some(c) = input.try_capture(&RE) {
            let name_group = c.name("spaces").or(c.name("name")).unwrap();
            let name = name_group.as_str().to_string();

            if let Some(post) = c.name("post") {
                // matched a "normal" entity
                input.move_forward(name_group.end());
                let used_brackets = post.as_str() == "{}";
                // skip the brackets (if present)
                if used_brackets {
                    input.move_forward(2);
                }

                let end = input.cursor - 1;
                let post_blank = input.skip_whitespace();

                Ok(Entity {
                    shared_behavior_data: SharedBehaviorData {
                        span: Span::new(start, end),
                        post_blank,
                    },
                    name,
                    used_brackets,
                })
            } else {
                // matched a "_spaces" entity
                input.move_forward(name_group.end());

                let end = input.cursor - 1;
                let post_blank = input.skip_whitespace();

                Ok(Entity {
                    shared_behavior_data: SharedBehaviorData {
                        span: Span::new(start, end),
                        post_blank,
                    },
                    name,
                    used_brackets: false,
                })
            }
        } else {
            Err(ParseError)
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let brackets = if self.used_brackets { "{}" } else { "" };
        write!(f, r"\{}{}", self.name, brackets)
    }
}

#[test]
fn test_parse_spaces_entity() {
    let s = r"\_ ";
    let mut input = Input::new(s);
    let parsed = Entity::parse(&mut input).unwrap();

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
    assert_eq!(input.cursor, 3);
    assert_eq!(parsed.to_string(), s);
}

#[test]
fn test_parse_entity() {
    let s = r"\name";
    let mut input = Input::new(s);
    let parsed = Entity::parse(&mut input).unwrap();

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
    assert_eq!(input.cursor, 5);
    assert_eq!(parsed.to_string(), s);
}

#[test]
fn test_parse_entity_with_brackets() {
    let s = r"\name{}";
    let mut input = Input::new(s);
    let parsed = Entity::parse(&mut input).unwrap();

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
    assert_eq!(input.cursor, 7);
    assert_eq!(parsed.to_string(), s);
}

#[test]
fn test_parse_entity_with_brackets_and_post_blanks() {
    let s = "\\name{}\t\t\t \t";
    let mut input = Input::new(s);
    let parsed = Entity::parse(&mut input).unwrap();

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
    assert_eq!(input.cursor, 12);
    assert_eq!(parsed.to_string(), r"\name{}");
}
