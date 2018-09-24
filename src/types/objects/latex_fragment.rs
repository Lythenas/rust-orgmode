use super::*;

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

