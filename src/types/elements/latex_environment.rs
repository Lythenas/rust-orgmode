use super::*;

/// A LaTeX environment.
///
/// # Semantics
///
/// This will be treated as accordingly when exporting with LaTeX. Otherwise it will be treated
/// as plain text.
///
/// # Syntax
///
/// ```text
/// \begin{ENVIRONMENT}
/// CONTENTS
/// \end{ENVIRONMENT}
/// ```
///
/// `ENVIRONMENT` can contain any alpha-numeric character and asterisks. Usually the asterisk
/// is only at the end.
///
/// `CONTENT` can be anything except `\end{ENVIRONMENT}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LatexEnvironment {
    affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    /// Contains everything including `\begin...` and `\end`.
    pub value: String,
}
