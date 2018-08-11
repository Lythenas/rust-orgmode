use failure::Error;
use nom::types::CompleteStr;
use std::convert::TryInto;

use *;

/// Parses the stars at the beginning of the line to their count.
named!(level<CompleteStr, u8, Error>,
    to_failure!(map_res!(
        take_while1!(|c| c == '*'),
        |s: CompleteStr| (*s).len().try_into()
    ))
);

/// Parses the keyword at the beginning of the headline (after the stars).
named!(keyword<CompleteStr, State, Error>,
    to_failure!(map_opt!(
        take_until!(" "),
        to_keyword
    ))
);

/// Converts the string to a keyword.
fn to_keyword(s: CompleteStr) -> Option<State> {
    // TODO make this more dynamic
    match *s {
        "TODO" => Some(State::Todo(String::from(*s))),
        "DONE" => Some(State::Done(String::from(*s))),
        _ => None,
    }
}

/// Parses the priority of the headline.
named!(priority<CompleteStr, Priority, Error>,
    to_failure!(map_res!(
        to_failure!(do_parse!(
            tag!("[#") >>
            prio: take!(1) >>
            tag!("]") >>
            (prio)
        )),
        |s: CompleteStr| (*s).parse()
    ))
);

named!(title<CompleteStr, String, Error>,
    to_failure!(map!(
        recognize!(
            fold_many0!(
                verify!(
                    alt_complete!(take!(1) | eof!()),
                    // TODO make this not consume the tags
                    |s: CompleteStr| (*s).len() != 0 && (*s) != "\n"
                ),
                (),
                |acc: (), _| acc
            )
        ),
        |s: CompleteStr| String::from(*s)
    ))
);

/// Parses the tags of a headline.
named!(tags<CompleteStr, Vec<String>, Error>,
    to_failure!(delimited!(
        tag!(":"),
        separated_list_complete!(
            tag!(":"),
            map!(
                take_until!(":"),
                |s: CompleteStr| String::from(*s)
            )
        ),
        tag!(":")
    ))
);

/// Parses a section.
///
/// Currently just takes all input until a new headline begins..
named!(section<CompleteStr, Section, Error>,
    to_failure!(map!(
        recognize!(
            fold_many0!(
                verify!(
                    // TODO maybe matching \n* is not the best,
                    // also take!(1) is no good here
                    alt_complete!(take_until!("\n*") | take!(1) | eof!()),
                    |s: CompleteStr| (*s).len() != 0 && !(*s).ends_with("\n*")
                ),
                (),
                |acc: (), _| acc
            )
        ),
        |s: CompleteStr| Section::new(*s)
    ))
);

named!(headline<CompleteStr, Headline, Error>,
    to_failure!(do_parse!(
        level: level >>
        keyword: opt!(preceded!(to_failure!(tag!(" ")), keyword)) >>
        priority: opt!(preceded!(to_failure!(tag!(" ")), priority)) >>
        to_failure!(tag!(" ")) >>
        title: title >>
        // TODO parse tags
        //to_failure!(tag!(" ")) >>
        //tags: tags >>
        to_failure!(eof!()) >> // TODO fix this
        (Headline::new(level, keyword, priority, title, Vec::new()))
    ))
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headline() {
        assert_eq!(
            headline(CompleteStr("* Headline without keyword and priority")).ok(),
            Some((
                CompleteStr(""),
                Headline::new(
                    1,
                    None,
                    None,
                    "Headline without keyword and priority",
                    Vec::new()
                )
            ))
        );
        assert_eq!(
            headline(CompleteStr(
                "* TODO [#A] Headline with keyword and priority"
            )).ok(),
            Some((
                CompleteStr(""),
                Headline::new(
                    1,
                    Some(State::Todo("TODO".into())),
                    Some(Priority::A),
                    "Headline with keyword and priority",
                    Vec::new()
                )
            ))
        );
        /*assert_eq!(
            headline(CompleteStr(
                "* TODO [#A] Headline with keyword and priority :tag1:tag2:"
            )).ok(),
            Some((
                CompleteStr(""),
                Headline::new(
                    1,
                    Some(State::Todo("TODO".into())),
                    Some(Priority::A),
                    "Headline with keyword and priority",
                    vec!["tag1".into(), "tag2".into()]
                )
            ))
        );*/
    }

    #[test]
    fn test_level() {
        assert_eq!(
            level(CompleteStr("***")).ok(),
            Some((
                CompleteStr(""),
                3
            ))
        );
        assert_eq!(
            level(CompleteStr("***** Title here")).ok(),
            Some((
                CompleteStr(" Title here"),
                5
            ))
        );
    }

    #[test]
    fn test_section() {
        assert_eq!(
            section(CompleteStr("Section content,\n...\nmore...")).ok(),
            Some((
                CompleteStr(""),
                Section::new("Section content,\n...\nmore...")
            ))
        );
        assert_eq!(
            section(CompleteStr(
                "Section content,\n...\nmore...\n\n** New headline"
            )).ok(),
            Some((
                CompleteStr("\n** New headline"),
                Section::new("Section content,\n...\nmore...\n")
            ))
        );
    }

    #[test]
    fn test_tags() {
        assert_eq!(
            tags(CompleteStr(":tag1:tag2:tag3:")).ok(),
            Some((
                CompleteStr(""),
                vec!["tag1".into(), "tag2".into(), "tag3".into()]
            ))
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(
            priority(CompleteStr("[#A]")).ok(),
            Some((
                CompleteStr(""),
                Priority::A
            ))
        );
        assert_eq!(
            priority(CompleteStr("[#Z] Headline")).ok(),
            Some((
                CompleteStr(" Headline"),
                Priority::Z
            ))
        );
    }

    #[test]
    fn test_keyword() {
        assert_eq!(
            keyword(CompleteStr("TODO ")).ok(),
            Some((
                CompleteStr(" "),
                State::Todo("TODO".into())
            ))
        );
        assert_eq!(
            keyword(CompleteStr("DONE Headline")).ok(),
            Some((
                CompleteStr(" Headline"),
                State::Done("DONE".into())
            ))
        );
    }

    #[test]
    fn test_title() {
        assert_eq!(
            title(CompleteStr("This is a test title.")).ok(),
            Some((CompleteStr(""), String::from("This is a test title.")))
        );
        assert_eq!(
            title(CompleteStr(
                "This is a test title.\nThis is not part of the title anymore."
            )).ok(),
            Some((
                CompleteStr("\nThis is not part of the title anymore."),
                String::from("This is a test title.")
            ))
        );
    }
}
