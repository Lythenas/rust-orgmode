use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subscript {
    pub used_brackets: bool,
    pub content: SecondaryString<StandardSet>,
}
