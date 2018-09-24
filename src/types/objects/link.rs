use super::*;

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

