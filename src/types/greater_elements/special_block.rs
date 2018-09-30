use super::*;
use crate::parsing::{Context, Parse, ParseError, Parser};
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
#[derive(
    Element, HasContent, GreaterElement, HasAffiliatedKeywords, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct SpecialBlock {
    shared_behavior_data: SharedBehaviorData,
    affiliated_keywords_data: Spanned<AffiliatedKeywords>,
    content_data: ContentData<String>,
    pub name: String,
    // hiddenp: bool
}

impl Parse for SpecialBlock {
    fn parse(parser: &mut Parser) -> Result<SpecialBlock, ParseError> {
        lazy_static! {
            static ref RE_START: Regex =
                Regex::new(r"(?m)\A^(?P<indentation>[ \t]*)#\+BEGIN_(?P<name>\S+)[ \t]*\n")
                    .unwrap();
        }

        fn make_re_end<'a>(_context: &Context, captures: &regex::Captures) -> Regex {
            let name = captures.name("name").unwrap();
            Regex::new(&format!(
                r"(?m)\A^(?P<indentation>[ \t]*)#\+END_{}\n?",
                regex::escape(name.as_str())
            ))
            .unwrap()
        }

        fn collect_data(
            context: &mut Context,
            captures: &regex::Captures<'_>,
        ) -> Result<String, ()> {
            let _indentation = captures.name("indentation").unwrap();
            let name = captures.name("name").unwrap();

            context.move_cursor_forward(captures.get(0).unwrap().end());

            Ok(name.as_str().into())
        }

        fn from_collected_data(
            name: String,
            shared_behavior_data: SharedBehaviorData,
            affiliated_keywords_data: Spanned<AffiliatedKeywords>,
            content_data: ContentData<String>,
        ) -> Result<SpecialBlock, !> {
            Ok(SpecialBlock {
                shared_behavior_data,
                affiliated_keywords_data,
                content_data,
                name,
            })
        }

        parser.parse_block_with_dynamic_end(
            &RE_START,
            make_re_end,
            collect_data,
            from_collected_data,
        )
    }
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
    use crate::parsing::Input;

    #[test]
    fn test_drawer_empty() {
        let s = "#+BEGIN_something\n#+END_something";
        let mut parser = Input::new(s).into();
        let parsed = SpecialBlock::parse(&mut parser).unwrap();

        assert_eq!(
            parsed,
            SpecialBlock {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(0, 33),
                    post_blank: 0,
                },
                affiliated_keywords_data: Spanned::new(
                    Span::new(0, 0),
                    AffiliatedKeywords::default()
                ),
                content_data: ContentData {
                    span: Span::new(18, 18),
                    content: Vec::default(),
                },
                name: "something".to_string(),
            }
        );
        assert_eq!(parser.cursor_pos(), 33);
        assert_eq!(parsed.to_string(), s);
    }

    // TODO add more tests once content parsing is implemented
}
