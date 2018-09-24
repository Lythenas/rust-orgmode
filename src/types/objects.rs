//! Contains all objects.

use super::parsing::{Input, Parse, ParseError};
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
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    shared_behavior_data: SharedBehaviorData,
    pub name: String,
    /// True if the entity ended with `{}`.
    pub used_brackets: bool,
}

fn build_regex(pattern: &str) -> Regex {
    use regex::RegexBuilder;
    RegexBuilder::new(pattern).multi_line(true).build().unwrap()
}

impl Parse for Entity {
    fn parse(input: &mut Input) -> Result<Self, ParseError> {
        // TODO maybe combine into one regex for a little less code below
        lazy_static! {
            static ref SPACES_RE: Regex = build_regex(r"\A\\(_ +)");
            static ref OTHER_RE: Regex =
                build_regex(r"\A\\(there4|sup[123]|frac[13][24]|[[:alpha:]]+)($|\{}|[^[:alpha:]])");
        }

        if let Some(m) = input.try_match(&SPACES_RE) {
            Ok(Entity {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(m.start(), input.cursor - 1),
                    post_blank: 0,
                },
                name: "".to_string(),
                used_brackets: false,
            })
        } else if let Some(m) = input.try_capture(&OTHER_RE) {
            let name = m.get(1).unwrap();
            let post = m.get(2).unwrap();
            let used_brackets = post.as_str() == "{}";
            if !used_brackets && !post.as_str().is_empty() {
                input.backup_cursor(1);
            }
            Ok(Entity {
                shared_behavior_data: SharedBehaviorData {
                    span: Span::new(name.start() - 1, input.cursor - 1),
                    post_blank: 0,
                },
                name: name.as_str().to_string(),
                used_brackets,
            })
        } else {
            Err(ParseError)
        }
    }
}

#[test]
fn test_parse_spaces_entity() {
    let s = r"\_ ";
    let mut input = Input::new(s);
    assert_eq!(
        Entity::parse(&mut input).unwrap(),
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
}

#[test]
fn test_parse_entity() {
    let s = r"\name";
    let mut input = Input::new(s);
    assert_eq!(
        Entity::parse(&mut input).unwrap(),
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
}

#[test]
fn test_parse_entity_with_brackets() {
    let s = r"\name{}";
    let mut input = Input::new(s);
    assert_eq!(
        Entity::parse(&mut input).unwrap(),
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
}

/// An export snippet.
///
/// # Semantics
///
/// These snippets are only exported in the specified format. E.g. there can be an export
/// snippet that is only exported in html.
///
/// # Syntax
///
/// ```text
/// @@BACKEND:VALUE@@
/// ```
///
/// `BACKEND` can contain any alpha-numeric character and hyphens.
///
/// `VALUE` can contain anything but the `@@` string.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportSnippet {
    shared_behavior_data: SharedBehaviorData,
    pub backend: String,
    pub value: String,
}

/// A footnote reference.
///
/// # Semantics
///
/// This is a reference (or link) to a [`greater_elements::FootnoteDefinition`].
///
/// # Syntax
///
/// Follows one of these patterns:
///
/// - normal footnote: `[fn:LABEL]`
/// - inline footnote: `[fn:LABEL:DEFINITION]`
///   can be references by other footnote
/// - anonymous footnote: `[fn::DEFINITION]`
///
/// `LABEL` can contain any word-constituent character, hyphens and underscores.
///
/// `DEFINITION` can contain any character. Opening and closing square brackets must be
/// balanced in it. It can contain the standard set of objects, even other footnote references.
/// Will be parsed as a secondary string and can contain the standard set of objects.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FootnoteReference {
    shared_behavior_data: SharedBehaviorData,
    pub kind: FootnoteReferenceKind,
}

/// The kind of a [`FootnoteReference`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FootnoteReferenceKind {
    Normal {
        label: String,
    },
    Inline {
        label: String,
        definition: SecondaryString<StandardSet>,
    },
    Anonymous {
        definition: SecondaryString<StandardSet>,
    },
}

/// An inline babe call.
///
/// # Semantics
///
/// Same as [`elements::BabelCall`] but inline.
///
/// # Syntax
///
/// ```text
/// call_NAME[HEADER](ARGUEMTNS)[HEADER]
/// ```
///
/// `NAME` can contain any character besides `(`, `[`, whitespace and newline.
///
/// `HEADER` can contain any characer besides `]` and newline.
///
/// `ARGUMENTS` can contain any character besides `)` and newline.
///
/// Both `HEADER`s are optional. But then there are also no square brackets.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InlineBabelCall {
    shared_behavior_data: SharedBehaviorData,
    pub call: String,
    pub inside_header: String,
    pub arguments: String,
    pub end_header: String,
}

/// An inline src block.
///
/// # Semantics
///
/// Same as [`elements::SrcBlock`] but inline.
///
/// # Syntax
///
/// ```text
/// src_LANG[OPTIONS]{BODY}
/// ```
///
/// `LANG` can contain any non-whitespace character.
///
/// `OPTIONS` and `BODY` can contain any character but a newline.
///
/// `OPTIONS` is optional. But then there are also not quote brackets.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InlineSrcBlock {
    shared_behavior_data: SharedBehaviorData,
    pub lang: String,
    pub value: String,
    pub options: String,
}

/// A LaTeX fragment.
///
/// # Semantics
///
/// # Syntax
///
/// Follows one of these patterns:
///
/// ```text
/// \NAME BRACKETS
/// \(CONTENTS\)
/// \[CONTENTS\]
/// $$CONTENTS$$
/// PRE$CHAR$POST
/// PRE$BORDER1 BODY BORDER2$POST
/// ```
///
/// `NAME` can contain any alphabetical character and can end with an asterisk. `NAME` must not
/// be in [`entities`] or the user defined `org_entities_user` variable otherwise it will
/// be parsed as a [`Entity`].
///
/// `BRACKETS` is optional and is not separated from `NAME` with whitespace. It can contain any
/// number of the following patterns (not separated by anything): `[CONTENTS1]`, `{CONTENTS2}`.
///
/// `CONTENTS1` and `CONTENTS2` can contain any character except `{`, `}` and newline.
/// Additionally `CONTENTS1` can't contain `[` and `]`.
///
/// `CONTENTS` can contain any character but the closing characters of the pattern used.
///
/// `PRE` is either the beginning of the line or any character except `$`.
///
/// `CHAR` is a non-whitspace character except `.`, `,`, `?`, `;`, `'` or `"`.
///
/// `POST` is any punctuation (including parantheses and quotes) or space character or the end
/// of the line.
///
/// `BORDER1` is any non-whitespace character except `.`, `,`, `;` and `$`.
///
/// `BODY` can contain any character except `$` and may not span over more than 3 lines.
///
/// `BORDER2` is any non-whitespace character except `.`, `,` and `$`.
///
/// [`entities`]: ../../entities/index.html
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LatexFragment {
    shared_behavior_data: SharedBehaviorData,
    /// Contains the entire parsed string, except the `PRE` and `POST` parts.
    pub value: String,
}

/// A line break.
///
/// # Semantics
///
/// Used to export a line break.
///
/// # Syntax
///
/// ```text
/// \\SPACE
/// ```
///
/// `SPACE` is zero or more whitespace characters followed by the end of line or end of
/// document.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineBreak {
    shared_behavior_data: SharedBehaviorData,
    pub spaces: u64,
}

/// A link.
///
/// # Semantics
///
/// This is either a link to an internal element or an external website or file.
///
/// # Syntax
///
/// There a 4 formats of links:
///
/// - radio link: `PRE1 RADIO POST1`
/// - angle link: `<PROTOCOL:PATH>`
/// - plain link: `PRE2 PROTOCOL:PATH2 POST2`
/// - bracket link: `[[PATH3]DESCRIPTION]`
///
/// `PRE1` and `POST1` are optional non-alpha-numeric characters.
///
/// `RADIO` is a string matched by a [`RadioTarget`].
///
/// `PROTOCOL` is a string in [`ORG_LINK_TYPES`].
///
/// `PATH` can contain any character except `]`, `<`, `>` and newline.
///
/// `PRE2` and `POST2` are optional non-word-constituent characters.
///
/// `PATH2` can contain any non-whitespace character except `(`, `)`, `<` and `>`. It must end
/// with a word-constituent character or any non-whitespace non-punctuation character followed
/// by `/`.
///
/// `PATH3` follows one of these patterns:
///
/// - file type: `FILENAME`, which is a absolute or relative file path
/// - protocol type: `PROTOCOL:PATH4` or `PROTOCOL://PATH4`
/// - id type: `id:ID`, where `ID` is a hexadecimal number optionally separated by hyphens
/// - custom-id type: `#CUSTOM-ID`
/// - coderef type: `(CODEREF)`
/// - fuzzy type: `FUZZY`
///
/// And can be followed by double colons (`::`) and another string containing anything except
/// `]`. Which will be used as the search option for following the link. See [`SearchOption`].
///
/// `PATH4`, `CUSTOM-ID`, `CODEREF` and `FIZZY` can contain any character except square
/// brackets.
///
/// `DESCRIPTION` is optional and must be enclosed with square brackets. It can contain any
/// character except square brackets. It is also parsed as a [`SecondaryString`] and can
/// contain any most object in the [`StandardSet`]. It also can't contain another
/// link unless it is a plain or angle link. (See [`LinkDescriptionSetOfObjects`]).
///
/// Whitespace and newlines in the link are replaced with a single space.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Link {
    shared_behavior_data: SharedBehaviorData,
    pub link: LinkFormat,
}

/// The format with the actual link data of a [`Link`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkFormat {
    Radio(String),
    Angle(String),
    Plain(String),
    /// The secondary string can contain: export snippet, inline babel call, inline src block,
    /// latex fragment, entity, macro, plain link, statistics cookie, sub/superscript,
    /// text markup.
    Bracket(
        LinkPath,
        Option<SearchOption>,
        Option<SecondaryString<LinkDescriptionSetOfObjects>>,
    ),
}

#[derive(AsRawString, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkDescriptionSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    ExportSnippet(objects::ExportSnippet),
    InlineBabelCall(objects::InlineBabelCall),
    InlineSrcBlock(objects::InlineSrcBlock),
    LatexFragment(objects::LatexFragment),
    /// Can contain links that are not plain or angle links. This will probably be ignored.
    Link(objects::Link),
    Macro(objects::Macro),
    StatisticsCookie(objects::StatisticsCookie),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    TextMarkup(objects::TextMarkup),
}

/// The kind and data of a bracket link in [`LinkFormat`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkPath {
    File(String),
    Id(String),
    CustomId(String),
    CodeRef(String),
    Fuzzy(String),
}

/// The search option of bracket [`LinkFormat`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchOption {
    /// Jump to line.
    Line(u64),
    /// Search for target (`<<TARGET>>`) or do a text search.
    Target(String),
    /// Restrict search to headlines.
    Headlines(String),
    /// Search for a custom id property (`:CUSTOM_ID: ...`).
    CustomId(String),
    /// Do a regular expression search.
    Regex(String),
}

/// A macro.
///
/// # Semantics
///
/// Macros are replaced by actual value when exporting.
///
/// Replacement values are defined in the variabel `org-export-global-macros` or document wide
/// with `#+MACRO: name     replacement text $1, $2 are arguments`. This macro can then be
/// called with `{{{name(arg1, arg2)}}` resulting in `replacement text arg1, arg2 are
/// arguments` when exporting.
///
/// The following macros are pre-defined:
///
/// - title
/// - author
/// - email
/// - date(FORMAT): refers to the `#+DATE` keyword. (FORMAT is optional)
/// - time(FORMAT): refers to the current date and time when exporting.
/// - modification-time(FORMAT, VC): refers to the last modified attribute of the file on disk.
///   If VC is given (e.g. `true`) then try to retrieve the modifiaction time from a version
///   control system but falls back to file attributes.
/// - input-file: refers to the filename of the exported file.
/// - property(PROPERTY-NAME, SEARCH-OPTION): returns the PROPERTY-NAME in the current element.
///   If SEARCH-OPTION refers to a remote entry that will be used instead.
/// - n(NAME, ACTION): Implements a custom counter by returning the number of times this macro
///   has been expanded so far. Using NAME creates different counters. If ACTION is `-` the
///   counter is not incremented. If ACTION is a number the counter is set to that value. If
///   ACTION is anything else the counter is reset to 1. You can reset the default timer by
///   leaving NAME empty.
///
///   TODO: Implement these macros when implementing exporting.
///
/// # Syntax
///
/// ```text
/// {{{NAME(ARGUMENTS)}}}
/// ```
///
/// `NAME` must atart with a letter and can be followed by any number of alpha-numeric
/// characters, hyphens and underscores.
///
/// `ARGUMENTS` can contain anything but the string `}}}`. Arguments are separated by commas.
/// Non-separating commas have to be escaped with a backslash character (if you want a literal
/// backslash directly before the comma it has to be escaped with another backslash).
///
/// Multiple whitespace and newline characters in `ARGUMENTS` are replaced by a single space.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Macro {
    shared_behavior_data: SharedBehaviorData,
    pub name: String,
    pub arguments: Vec<String>,
}

/// A target that is automatically linked to.
///
/// # Semantics
///
/// A radio target e.g. with the value `<<<My Target>>>` makes every occurrence of the text `my
/// target` (case is ignored) in the document link to the target.
///
/// TODO I think this is only in the editor. Not sure how they are exported.
///
/// # Syntax
///
/// ```text
/// <<<TARGET>>>
/// ```
///
/// `TARGET` can contain any character except `<`, `>` and newline. It can't start or end with
/// a whitespace character. It will be parsed as a [`SecondaryString`].
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RadioTarget {
    shared_behavior_data: SharedBehaviorData,
    pub target: SecondaryString<StandardSet>,
}

#[derive(AsRawString, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RadioTargetSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    LatexFragment(objects::LatexFragment),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    TextMarkup(objects::TextMarkup),
}

/// A statistics cookie.
///
/// # Semantics
///
/// Used in [`Headline`]s, [`Inlinetask`]s and the first [`Item`] of [`PlainList`]s to represent
/// the amount of done tasks or checked items.
///
/// # Syntax
///
/// There are two kinds of cookies:
///
/// - percentage: `[PERCENT%]`
/// - number: `[NUM1/NUM2]`
///
/// `PERCENT`, `NUM1` and `NUM2` are numbers or an empty string.
///
/// [`Headline`]: `greater_elements::Headline`
/// [`Inlinetask`]: `greater_elements::Inlinetask`
/// [`Item`]: `greater_elements::Item`
/// [`PlainList`]: `greater_elements::PlainList`
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatisticsCookie {
    shared_behavior_data: SharedBehaviorData,
    pub cookie: CookieKind,
}

/// This is the kind and data of a [`StatisticsCookie`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CookieKind {
    Percent(Option<u32>),
    Number(Option<u32>, Option<u32>),
}

/// A subscript.
///
/// # Semantics
///
/// A subscript in the text.
///
/// # Syntax
///
/// ```text
/// CHAR_SCRIPT
/// ```
///
/// `CHAR` is any non-whitespace character.
///
/// `SCRIPT` can be `*` or any expression enclosed in parenthesis or curly brackets. It can
/// contain balanced parenthesis and curly brackets.
///
/// Or `SCRIPT` can collow the pattern:
///
/// ```text
/// SIGN CHARS FINAL
/// ```
///
/// `SIGN` is either a plus sign, a minus sign or an empty string.
///
/// `CHARS` in any number of alpha-numeric characters, comas, backslashes and dots or an empty
/// string.
///
/// `FINAL` is an alpha-numeric character.
///
/// There is no whitespace between `SIGN`, `CHARS` and `FINAL`.
///
/// TODO this is recursive object. figure out how to handle recursive objects because some can
/// only contain specific objects and therefore other recursive objects in them may contain
/// less objects than they can usually contain
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subscript {
    shared_behavior_data: SharedBehaviorData,
    pub used_brackets: bool,
    pub content: SecondaryString<StandardSet>,
}

/// A superscript.
///
/// # Semantics
///
/// A superscript in the text.
///
/// # Syntax
///
/// ```text
/// CHAR_SCRIPT
/// ```
///
/// See [`Subscript`].
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Superscript {
    shared_behavior_data: SharedBehaviorData,
    pub used_brackets: bool,
    pub content: SecondaryString<StandardSet>,
}

/// A table cell in a [`greater_elements::TableRow`].
///
/// # Semantics
///
/// The content of a table row.
///
/// # Syntax
///
/// ```text
/// CONTENTS SPACES |
/// ```
///
/// `CONTENTS` can contain any character except a vertical bar.
///
/// `SPACES` contains any number (including zero) of soace and tab characters. This is usually
/// used to align the table properly.
///
/// The final bar my be replaced with a newline character for the last cell in the row.
///
/// TODO recusrive object. can contain: export snippet, footnote reference, latex fragment,
/// entity, link, macro, radio target, sub/superscript, target, text markup, timestamp
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableCell {
    shared_behavior_data: SharedBehaviorData,
    pub content: ContentData<TableCellSetOfObjects>,
}

/// Table cells can't contain [`InlineBabelCall`], [`InlineSrcBlock`] because formulas are
/// possible. Also they can't contain [`LineBreak`] and [`StatisticsCookie`].
#[derive(AsRawString, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableCellSetOfObjects {
    RawString(String),
    Entity(objects::Entity),
    ExportSnippet(objects::ExportSnippet),
    FootnoteReference(objects::FootnoteReference),
    LatexFragment(objects::LatexFragment),
    Link(objects::Link),
    Macro(objects::Macro),
    RadioTarget(objects::RadioTarget),
    Subscript(objects::Subscript),
    Superscript(objects::Superscript),
    Target(objects::Target),
    TextMarkup(objects::TextMarkup),
    Timestamp(objects::Timestamp),
}

/// A target.
///
/// # Semantics
///
/// Used to link to internal objects that can't be assigned affiliated keywords. E.g. list
/// items.
///
/// See fuzzy [`Link`]s.
///
/// # Syntax
///
/// ```text
/// <<TARGET>>
/// ```
///
/// `TARGET` can contain any character except `<`, `>` and newline. It can't start or end with
/// a whitespace character. It will not be parsed.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    shared_behavior_data: SharedBehaviorData,
    pub target: String,
}

/// A text formatter.
///
/// # Semantics
///
/// Formats text according to the marker used:
///
/// - `*bold*`
/// - `/italic/`
/// - `_underline_`
/// - `+strike through+`
/// - `~code~`
/// - `=verbatim=`
///
/// # Syntax
///
/// ```text
/// PRE MARKER BORDER BODY BORDER MARKER POST
/// ```
///
/// Not separated by any whitespace.
///
/// `PRE` is one of `-`, whitespace, `(`, `'`,`"`, `{` or beginning of line.
///
/// `BORDER` is anything but whitespace, `,`, `'` and `"`.
///
/// `MARKER` is one of the markers specified in [semantics][#Semantics].
///
/// `BODY` can contain any character but may not span over more than 3 lines.
///
/// `POST` is one of `-`, whitespace, `.`, `,`, `:`, `!`, `?`, `;`, `'`, `"`, `)`, `}`, `[` or
/// end of line.
///
/// The part `BORDER BODY BORDER` is parsed as a [`SecondaryString`] and can contain the
/// standard set of objects when the markup is bold, italic, strike through or udnerline. The
/// content of verbatim and code is not parsed.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextMarkup {
    shared_behavior_data: SharedBehaviorData,
    pub kind: TextMarkupKind,
}

/// The kind and content of a [`TextMarkup`] object.
///
/// Only code and verbatim can't contain other objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextMarkupKind {
    // TODO maybe make these actual different types instead of an enum
    Bold(SecondaryString<StandardSet>),
    Italic(SecondaryString<StandardSet>),
    Underline(SecondaryString<StandardSet>),
    StrikeThrough(SecondaryString<StandardSet>),
    Code(String),
    Verbatim(String),
}

pub use self::timestamp::Timestamp;

/// Contains the [`Timestamp`][`timestamp::Timestamp`] object and all structs used by it.
pub mod timestamp {
    use super::*;

    /// A timestamp.
    ///
    /// # Semantics
    ///
    /// Timestamps are used in [`elements::Clock`] and [`elements::Planning`] and can occur in normal text.
    ///
    /// They represent a date and time and can be either active or inactive. Usually inactive means
    /// that the event is already over or represents the date the event has been dealt with.
    ///
    /// # Syntax
    ///
    /// Follows one of the patterns:
    ///
    /// - diary sexp: `<%%(SEXP)>`
    /// - active: `<INNER>`
    /// - inactive: `[INNER]`
    /// - active range: `<INNER>--<INNER>` or `<DATE TIME-TIME REPEATERORDELAY>`
    /// - inactive range: `[INNER]--[INNER]` or `[DATE TIME-TIME REPEATERORDELAY]`
    ///
    /// `SEXP` can contain any character except `>` and newline.
    ///
    /// `INNER` is the pattern `DATE TIME REPEATERORDERLAY`.
    ///
    /// `DATE` follows the pattern `YYYY-MM-DD DAYNAME`. Where `Y`, `M` and `D` are digits
    /// (`0`-`9`). `DAYNAME` is optional and can contain any non-whitespace character except `+`,
    /// `-`, `]`, `>`, digits and newlines. Usually it is the three letter name of the weekday.
    ///
    /// `TIME` follows the pattern `HH:MM`. Where `H` and `M` are digits. The first `H` can be
    /// omitted.
    ///
    /// `REPEATERORDELAY` follows the pattern `MARK VALUE UNIT` where `MARK` is one of `+`, `++`,
    /// `.+`, `-` or `--` for the repeat or delay strategy. `VALUE` is a (positive) number. `UNIT`
    /// is one of `h`, `d`, `w`, `m` or `y`.
    ///
    /// There can be two `REPEATERORYEAR` in the timestamp. One as a repeater and on as a warning
    /// delay.
    #[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Timestamp {
        shared_behavior_data: SharedBehaviorData,
        pub kind: TimestampKind,
    }

    impl Timestamp {
        pub fn timestamp_start(&self) -> Option<(&Date, Option<&Time>)> {
            use self::TimestampKind::*;
            use self::TimestampRange::*;

            match &self.kind {
                DiarySexp(_) => None,
                Single(_, TimestampData { date, time, .. })
                | Range(_, DateRange(TimestampData { date, time, .. }, ..)) => {
                    Some((&date, time.as_ref()))
                }
                Range(_, TimeRange(TimestampDataWithTime { date, time, .. }, ..)) => {
                    Some((&date, Some(&time)))
                }
            }
        }
        pub fn timestamp_end(&self) -> Option<(&Date, Option<&Time>)> {
            use self::TimestampKind::*;
            use self::TimestampRange::*;

            match &self.kind {
                DiarySexp(_) => None,
                Single(_, TimestampData { date, time, .. }) => Some((&date, time.as_ref())),
                Range(_, TimeRange(TimestampDataWithTime { date, .. }, time)) => {
                    Some((&date, Some(&time)))
                }
                Range(_, DateRange(_, TimestampData { date, time, .. })) => {
                    Some((&date, time.as_ref()))
                }
            }
        }
        pub fn repeater(&self) -> Option<&Repeater> {
            use self::TimestampKind::*;
            use self::TimestampRange::*;

            match &self.kind {
                DiarySexp(_) => None,
                Single(_, TimestampData { repeater, .. })
                | Range(_, TimeRange(TimestampDataWithTime { repeater, .. }, _))
                | Range(_, DateRange(TimestampData { repeater, .. }, _)) => repeater.as_ref(),
            }
        }
        pub fn warning(&self) -> Option<&Warning> {
            use self::TimestampKind::*;
            use self::TimestampRange::*;

            match &self.kind {
                DiarySexp(_) => None,
                Single(_, TimestampData { warning, .. })
                | Range(_, TimeRange(TimestampDataWithTime { warning, .. }, _))
                | Range(_, DateRange(TimestampData { warning, .. }, _)) => warning.as_ref(),
            }
        }
    }

    /// The kind and date for a [`Timestamp`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TimestampKind {
        DiarySexp(String),
        Single(TimestampStatus, TimestampData),
        Range(TimestampStatus, TimestampRange),
    }

    /// The status of a [`Timestamp`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TimestampStatus {
        /// Timestamp in angle brackets (`<...>`).
        Active,
        /// Timestamp in square brackets (`[...]`).
        Inactive,
    }

    /// The data for a [`TimestampKind`] with optional [`Time`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TimestampData {
        pub date: Date,
        pub time: Option<Time>,
        pub repeater: Option<Repeater>,
        pub warning: Option<Warning>,
    }

    /// A date.
    ///
    /// This is a wrapper around [`chrono::NaiveDate`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Date(NaiveDate);

    /// A time.
    ///
    /// This is a wrapper around [`chrono::NaiveTime`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Time(NaiveTime);

    /// The repeater of a timestamp.
    ///
    /// See [`TimestampData`] and [`TimestampDataWithTime`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Repeater {
        pub period: TimePeriod,
        pub strategy: RepeatStrategy,
    }

    /// The warning delay of a timestamp.
    ///
    /// See [`TimestampData`] and [`TimestampDataWithTime`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Warning {
        pub delay: TimePeriod,
        pub strategy: WarningStrategy,
    }

    /// The time period (with unit) of a [`Repeater`] or [`Warning`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TimePeriod {
        pub value: u32,
        pub unit: TimeUnit,
    }

    /// The strategy of a [`Repeater`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum RepeatStrategy {
        /// Add the repeat duration to the task date once.
        Cumulative,
        /// Add the repeat duration to the task date until the date is in the
        /// future (but at leas once).
        CatchUp,
        /// Add the repeat duration to the current time.
        Restart,
    }

    /// The strategy of a [`Warning`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum WarningStrategy {
        /// Warns for all (repeated) date. Represented as `-` in the org file.
        All,
        /// Warns only for the first date. Represented as `--` in the org file.
        First,
    }

    /// The unit of a [`TimePeriod`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TimeUnit {
        Year,
        Month,
        Week,
        Day,
        Hour,
    }

    /// The data for a timestamp range.
    ///
    /// See [`TimestampKind`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TimestampRange {
        /// `<DATE TIME-TIME REPEATER-OR-DELAY>` or
        /// `[DATE TIME-TIME REPEATER-OR-DELAY]`
        TimeRange(TimestampDataWithTime, Time),
        /// `<DATE TIME REPEATER-OR-DELAY>--<DATE TIME REPEATER-OR-DELAY>` or
        /// `[DATE TIME REPEATER-OR-DELAY]--[DATE TIME REPEATER-OR-DELAY]`
        DateRange(TimestampData, TimestampData),
    }

    /// The data for a timestamp with a time.
    ///
    /// See [`TimestampRange`].
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TimestampDataWithTime {
        pub date: Date,
        pub time: Time,
        pub repeater: Option<Repeater>,
        pub warning: Option<Warning>,
    }

}
