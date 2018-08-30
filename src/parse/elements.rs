use parse::{OrgInput, OrgResult};
use Keyword;

/// Parses a single keyword.
///
/// A keyword has the form:
///
/// ```text
/// #+KEY: VALUE
/// ```
///
/// `KEY` can contain any non-whitespace character, but it cannot be
/// equal to `CALL` or any affiliated keyword.
///
/// This rule is ignored and should be checked either later or by the user.
pub fn single_keyword(i: OrgInput) -> OrgResult<Keyword> {
    to_failure!(
        i,
        do_parse!(
            tag!("#+")
                >> key: take_until!(": ")
                >> tag!(": ")
                >> value: take_until_or_eof!("\n")
                >> (Keyword::new(*key, *value))
        )
    )
}

/// Parses all sucessive keywords.
///
/// See: [`single_keyword`]
pub fn keywords(i: OrgInput) -> OrgResult<Vec<Keyword>> {
    to_failure!(i, separated_list!(to_failure!(tag!("\n")), single_keyword))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteStr;

    #[test]
    fn test_single_keyword() {
        assert_eq!(
            single_keyword(CompleteStr("#+some_key: some_value")).ok(),
            Some((CompleteStr(""), Keyword::new("some_key", "some_value")))
        );
        assert_eq!(
            single_keyword(CompleteStr("#+some_key:more:: some_value")).ok(),
            Some((
                CompleteStr(""),
                Keyword::new("some_key:more:", "some_value")
            ))
        );
    }

    #[test]
    fn test_keywords() {
        assert_eq!(
            keywords(CompleteStr("#+key1: value1\n#+key2: value2")).ok(),
            Some((
                CompleteStr(""),
                vec![
                    Keyword::new("key1", "value1"),
                    Keyword::new("key2", "value2")
                ]
            ))
        );
        assert_eq!(
            keywords(CompleteStr("#+key1: value1\n#+key2: value2\nnot a keyword")).ok(),
            Some((
                CompleteStr("\nnot a keyword"),
                vec![
                    Keyword::new("key1", "value1"),
                    Keyword::new("key2", "value2")
                ]
            ))
        );
    }
}
