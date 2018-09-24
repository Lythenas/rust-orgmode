use super::*;

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

