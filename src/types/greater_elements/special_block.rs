use super::*;
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
    content_data: ContentData<Spanned<String>>,
    pub name: String,
    // hiddenp: bool
}

use combine::error::ParseError;
use combine::parser::char::char;
use combine::parser::regex::{captures, find};
use combine::parser::repeat::take_until;
use combine::stream::state::{IndexPositioner, State};
use combine::stream::{FullRangeStream, Stream};
use combine::{choice, position, value, Parser};
use crate::parsing::shared_behavior_data;

//fn greater_element()

// fn content_data<'a, I, T>() -> impl Parser<Input = I, Output = ContentData<T>> + 'a
// where
//     I: Stream<Item = char, Range = &'a str, Position = usize> + FullRangeStream + 'a,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
//     T: Clone + 'a,
// {
//     value(ContentData::empty(Span::new(0, 0)))
// }
// 
// fn parse_until<'a, I, P1, P2, S>(
//     p1: P1,
//     p2: P2,
// ) -> impl Parser<Input = I, Output = P2::Output> + 'a
// where
//     I: Stream<Item = char, Range = &'a str, Position = usize> + FullRangeStream + 'a,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
//     P1: Parser<Input = I>,
//     P2: Parser<Input = State<&'a str, IndexPositioner>>,
// {
//     position().then(|position| {
//         take_until(p1).and_then(|content_str| {
//             p2.easy_parse(&content_str, State::with_positioner(IndexPositioner::new_with_position(
//                 position,
//             )))
//             .map(|(output, _)| output)
//         })
//     })
// }
// 
// fn special_block<'a, I: 'a>() -> impl Parser<Input = I, Output = SpecialBlock> + 'a
// where
//     I: Stream<Item = char, Range = &'a str, Position = usize> + FullRangeStream,
//     I::Error: ParseError<I::Item, I::Range, I::Position>,
// {
//     lazy_static! {
//         static ref RE_START: Regex = Regex::new(r"([ \t]*)#\+BEGIN_(\S+)[ \t]*\n").unwrap();
//     }
// 
//     shared_behavior_data(
//         captures(&*RE_START)
//             .map(|vec: Vec<&str>| vec[2].to_string())
//             .then(|name| {
//                 let re =
//                     Regex::new(&format!(r"([ \t]*)#\+END_{}\n?", regex::escape(&name))).unwrap();
//                 (value(name), parse_until(find(&re), content_data()))
//             }),
//     )
//     .map(
//         |(shared_behavior_data, (name, content_data))| SpecialBlock {
//             shared_behavior_data,
//             affiliated_keywords_data: Spanned::new(Span::new(0, 0), AffiliatedKeywords::default()),
//             content_data,
//             name,
//         },
//     )
// }

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

    // #[test]
    // fn test_drawer_empty() {
    //     let s = "#+BEGIN_something\n#+END_something";
    //     let mut parser = Input::new(s).into();
    //     let parsed = SpecialBlock::parse(&mut parser).unwrap();

    //     assert_eq!(
    //         parsed,
    //         SpecialBlock {
    //             shared_behavior_data: SharedBehaviorData {
    //                 span: Span::new(0, 33),
    //                 post_blank: 0,
    //             },
    //             affiliated_keywords_data: Spanned::new(
    //                 Span::new(0, 0),
    //                 AffiliatedKeywords::default()
    //             ),
    //             content_data: ContentData {
    //                 span: Span::new(18, 18),
    //                 content: Vec::default(),
    //             },
    //             name: "something".to_string(),
    //         }
    //     );
    //     assert_eq!(parser.cursor_pos(), 33);
    //     assert_eq!(parsed.to_string(), s);
    // }

    // TODO add more tests once content parsing is implemented
}
