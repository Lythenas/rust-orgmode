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
    to_failure!(map!(
        take_until!(" "),
        to_keyword
    ))
);

/// Converts the string to a keyword.
fn to_keyword(s: CompleteStr) -> State {
    // TODO make this more dynamic
    match *s {
        "" => State::None,
        "TODO" => State::Todo(String::from(*s)),
        "DONE" => State::Done(String::from(*s)),
        _ => State::Other(String::from(*s)),
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
    dbg_dmp!(to_failure!(map!(
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
    )))
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
        take_until!("\n*"),
        |s: CompleteStr| Section::new(*s)
    ))
);

named!(headline<CompleteStr, Headline, Error>,
    to_failure!(do_parse!(
        level: level >>
        to_failure!(tag!(" ")) >>
        keyword: opt!(keyword) >>
        to_failure!(tag!(" ")) >>
        priority: opt!(priority) >>
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
    fn test_title() {
        assert_eq!(
            title(CompleteStr("This is a test title.")).ok(),
            Some((
                CompleteStr(""),
                String::from("This is a test title.")
            ))
        );
    }
}
