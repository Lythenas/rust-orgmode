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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Macro {
    pub name: String,
    pub arguments: Vec<String>,
}
