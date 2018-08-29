use parse::{OrgInput, OrgResult, headline, section};
use {OrgFile};

/// Parses a complete org file.
///
/// Currently does not parse the global properties.
pub fn file(i: OrgInput) -> OrgResult<OrgFile> {
    to_failure!(i, do_parse!(
        // TODO keywords/metadata at the start of the file
        section: opt!(section) >>
        to_failure!(opt!(tag!("\n"))) >>
        headlines: separated_list!(
            to_failure!(tag!("\n")),
            headline
        ) >>
        (OrgFile::new(Vec::new(), section.unwrap_or_default(), headlines))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteStr;

    #[test]
    fn test_empty_file() {
        assert_eq!(
            file(CompleteStr("")).ok(),
            Some((CompleteStr(""), OrgFile::default()))
        );
    }
}
