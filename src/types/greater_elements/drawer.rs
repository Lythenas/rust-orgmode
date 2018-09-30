use super::*;
use crate::types::parsing::{Context, Parse, ParseError, Parser};
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

impl Parse for Drawer {
    fn parse(parser: &mut Parser) -> Result<Drawer, ParseError> {
        lazy_static! {
            static ref RE_START: Regex =
                Regex::new(r"(?m)\A^(?P<indentation>[ \t]*):(?P<name>[\w_-]+):[ \t]*\n").unwrap();
            static ref RE_END: Regex = Regex::new(r"(?m)\A^(?P<indentation>[ \t]*):END:").unwrap();
        }

        fn collect_data(
            context: &mut Context,
            captures: &regex::Captures<'_>,
        ) -> Result<String, ()> {
            let _indentation = captures.name("indentation").unwrap();
            let name = captures.name("name").unwrap();

            // TODO add indentation to context (not sure if this is important)

            context.move_cursor_forward(captures.get(0).unwrap().end());

            Ok(name.as_str().into())
        }

        fn from_collected_data(
            name: String,
            shared_behavior_data: SharedBehaviorData,
            affiliated_keywords_data: Spanned<AffiliatedKeywords>,
            content_data: ContentData<ElementSet>,
        ) -> Result<Drawer, !> {
            Ok(Drawer {
                shared_behavior_data,
                affiliated_keywords_data,
                content_data,
                name,
            })
        }

        parser.parse_block(&RE_START, &RE_END, collect_data, from_collected_data)
    }
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
    use crate::types::parsing::Input;

    #[test]
    fn test_drawer_empty() {
        let s = ":something:\n:END:";
        let mut parser = Input::new(s).into();
        let parsed = Drawer::parse(&mut parser).unwrap();

        assert_eq!(
            parsed,
            Drawer {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(0, 17),
                    post_blank: 0,
                },
                affiliated_keywords_data: Spanned::new(
                    Span::new(0, 0),
                    AffiliatedKeywords::default()
                ),
                content_data: ContentData {
                    span: Span::new(12, 12),
                    content: Vec::default(),
                },
                name: "something".to_string(),
            }
        );
        assert_eq!(parser.cursor_pos(), 17);
        assert_eq!(parsed.to_string(), s);
    }

    // TODO add more tests as soon as node property parsing is implemented
}
