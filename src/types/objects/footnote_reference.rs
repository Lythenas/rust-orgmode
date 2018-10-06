use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FootnoteReference {
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
