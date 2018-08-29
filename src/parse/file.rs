use parse::{OrgInput, OrgResult, headline, section};
use {OrgFile, Headline};

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
        (OrgFile::new(Vec::new(), section.unwrap_or_default(), fix_structure(headlines)))
    ))
}

/// Fixes the structure of the headlines.
///
/// Given a completely flat list of all headlines. Nest them correctly.
///
/// TODO remove calls to unwrap
fn fix_structure(flat: Vec<Headline>) -> Vec<Headline> {
    let mut iter = flat.into_iter();
    let mut result = Vec::new();

    let mut last = Vec::new();
    match iter.next() {
        Some(x) => {
            last.push(x);
        },
        None => return result,
    }

    for current in iter {
        if current.level() > last.last().unwrap().level() {
            // add to last's sub_headlines
            last.last_mut().unwrap().sub_headlines_mut().push(current)
        } else {
            // pop from last and add to result
            result.push(last.pop().unwrap());
            // add current to last
            last.push(current);
        }
    }

    result.append(&mut last);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteStr;
    use {Headline, Section};

    #[test]
    fn test_empty_file() {
        assert_eq!(
            file(CompleteStr("")).ok(),
            Some((CompleteStr(""), OrgFile::default()))
        );
    }

    #[test]
    fn test_simple_file() {
        let input = "* Heading 1
** Heading 1.1
** Heading 1.2
** Heading 1.3
* Heading 2
* Heading 3";
        assert_eq!(
            file(CompleteStr(input)).ok(),
            Some((
                CompleteStr(""),
                OrgFile::new(
                    Vec::new(),
                    Section::new(""),
                    vec![
                        Headline::new(1, "Heading 1")
                            .and_sub_headlines(vec![
                                Headline::new(2, "Heading 1.1"),
                                Headline::new(2, "Heading 1.2"),
                                Headline::new(2, "Heading 1.3"),
                            ]),
                        Headline::new(1, "Heading 2"),
                        Headline::new(1, "Heading 3"),
                    ]
                )
            ))
        );
    }
}
