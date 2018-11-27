use super::*;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    pub name: String,
    /// True if the entity ended with `{}`.
    pub used_brackets: bool,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let brackets = if self.used_brackets { "{}" } else { "" };
        write!(f, r"\{}{}", self.name, brackets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
