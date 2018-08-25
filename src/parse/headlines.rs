use failure::Error;
use nom::types::CompleteStr;
use std::convert::TryInto;

use *;

named!(#[doc = "Parses the stars at the beginning of the line to their count."],
level<CompleteStr, u8, Error>,
    to_failure!(map_res!(
        take_while1!(|c| c == '*'),
        |s: CompleteStr| (*s).len().try_into()
    ))
);

/// 
named!(#[doc = "Parses the keyword at the beginning of the headline (after the stars)."],
keyword<CompleteStr, State, Error>,
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

named!(#[doc = "Parses the priority of the headline."],
priority<CompleteStr, Priority, Error>,
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

named!(#[doc = "Parses the title of a headline."],
title<CompleteStr, String, Error>,
    to_failure!(map!(
        // TODO make this not consume the tags
        take_until_or_eof!("\n"),
        |s: CompleteStr| String::from(*s)
    ))
);

named!(#[doc = "Parses the tags of a headline."],
tags<CompleteStr, Vec<String>, Error>,
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

named!(#[doc = "
Parses a section.

Currently just takes all input until a new headline begins."],
section<CompleteStr, Section, Error>,
    to_failure!(map!(
        // TODO maybe matching \n* is not the best,
        take_until_or_eof!("\n*"),
        |s: CompleteStr| Section::new(*s)
    ))
);

named!(#[doc = "Parses a planning line. (optional line directly under the headline)"],
planning<CompleteStr, Planning, Error>,
    map_res!(
        permutation!(
            opt!(delimited!(
                to_failure!(tag!("DEADLINE: ")),
                timestamp,
                to_failure!(opt!(tag!(" ")))
            )),
            opt!(delimited!(
                to_failure!(tag!("SCHEDULED: ")),
                timestamp,
                to_failure!(opt!(tag!(" ")))
            )),
            opt!(delimited!(
                to_failure!(tag!("CLOSED: ")),
                timestamp,
                to_failure!(opt!(tag!(" ")))
            ))
        ),
        to_planning
    )
);

/// Converts a deadline, scheduled and closed timestamp (all optional) to a [`Planning`] object.
fn to_planning(
    (deadline, scheduled, closed): (Option<Timestamp>, Option<Timestamp>, Option<Timestamp>),
) -> Result<Planning, ()> {
    if deadline.is_none() && scheduled.is_none() && closed.is_none() {
        Err(())
    } else {
        Ok(Planning::default()
            .and_opt_deadline(deadline)
            .and_opt_scheduled(scheduled)
            .and_opt_closed(closed))
    }
}

named!(#[doc = "
Parses a property drawer with node properties.

TODO (for later) make this recognize an indented property drawer"],
property_drawer<CompleteStr, PropertyDrawer, Error>,
    do_parse!(
        to_failure!(tag!(":PROPERTIES:\n")) >>
        list: opt!(separated_list!(to_failure!(tag!("\n")), node_property)) >>
        to_failure!(opt!(tag!("\n"))) >>
        to_failure!(tag!(":END:")) >>
        (PropertyDrawer::new(list.unwrap_or_default()))
    )
);

named!(#[doc = "
Parses a single node property of a property drawer.

Can be of the following formats:

- `:NAME: VALUE`
- `:NAME+: VALUE`
- `:NAME:`
- `:NAME+:`

**Note:** `NAME` can't be `END`."],
node_property<CompleteStr, NodeProperty, Error>,
    to_failure!(do_parse!(
        name: verify!(
            delimited!(tag!(":"), take_while!(|c| c != ':'), tag!(":")),
            |name: CompleteStr| *name != "END"
        ) >>
        value: opt!(preceded!(tag!(" "), take_while!(|c| c != '\n'))) >>
        (to_node_property(*name, value.map(|v| *v)))
    ))
);

/// Converts a name and optional value to a [`NodeProperty`].
fn to_node_property(name: &str, value: Option<&str>) -> NodeProperty {
    match value {
        Some(value) if !value.is_empty() => if name.ends_with('+') {
            NodeProperty::KeyPlusValue(name[..name.len()-1].to_string(), value.to_string())
        } else {
            NodeProperty::KeyValue(name.to_string(), value.to_string())
        },
        None | Some(_) => if name.ends_with('+') {
            NodeProperty::KeyPlus(name[..name.len()-1].to_string())
        } else {
            NodeProperty::Key(name.to_string())
        },
    }
}

named!(#[doc = "
Parses a complete headline.

Has the format:

```text
STARS KEYWORD PRIORITY TITLE TAGS
PLANNING
PROPERTY_DRAWER
SECTION
```

Where `KEYWORD`, `PRIORITY`, `TAGS`, `PLANNING`, `PROPERTY_DRAWER` and `SECTION` are optional.

`TAGS` is not yet implemented.

For the formats of the items see:

- `STARS`: [`level`]
- `KEYWORD`: [`keyword`]
- `PRIORITY`: [`priority`]
- `TITLE`: [`title`]
- `TAGS`: [`tags`]
- `PLANNING`: [`planning`]
- `PROPERTY_DRAWER`: [`property_drawer`]
- `SECTION`: [`section`]"],
pub headline<CompleteStr, Headline, Error>,
    dbg!(to_failure!(do_parse!(
        level: level >>
        keyword: opt!(preceded!(to_failure!(tag!(" ")), keyword)) >>
        priority: opt!(preceded!(to_failure!(tag!(" ")), priority)) >>
        to_failure!(tag!(" ")) >>
        title: title >>
        // TODO parse tags
        //to_failure!(tag!(" ")) >>
        //tags: tags >>
        // TODO fix: headline without planning and property_drawer needs two newlines
        planning: opt!(preceded!(to_failure!(tag!("\n")), planning)) >>
        property_drawer: opt!(preceded!(to_failure!(tag!("\n")), property_drawer)) >>
        section: opt!(preceded!(to_failure!(dbg!(tag!("\n"))), section)) >>
        to_failure!(opt!(tag!("\n"))) >>
        // TODO fix this
        //to_failure!(eof!()) >>
        (
            Headline::new(level, title)
                .and_opt_keyword(keyword)
                .and_opt_priority(priority)
                //.and_tags(tags)
                .and_planning(planning.unwrap_or_default())
                .and_property_drawer(property_drawer.unwrap_or_default())
                .and_opt_section(section.filter(|section| !section.is_empty()))
        )
    )))
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headline_with_section() {
        assert_eq!(
            headline(CompleteStr("* Headline\nThis is a section.")).ok(),
            Some((
                CompleteStr(""),
                Headline::new(1, "Headline")
                    .and_section(Section::new("This is a section."))
            ))
        );
    }

    #[test]
    fn test_headline() {
        assert_eq!(
            headline(CompleteStr("* Headline without keyword and priority")).ok(),
            Some((
                CompleteStr(""),
                Headline::new(1, "Headline without keyword and priority",)
            ))
        );
        assert_eq!(
            headline(CompleteStr(
                "* TODO [#A] Headline with keyword and priority"
            )).ok(),
            Some((
                CompleteStr(""),
                Headline::new(1, "Headline with keyword and priority",)
                    .and_keyword(State::Todo("TODO".into()))
                    .and_priority(Priority::A)
            ))
        );
        assert_eq!(
            headline(CompleteStr(
                "* Headline\n:PROPERTIES:\n:test_name:\n:END:"
            )).ok(),
            Some((
                CompleteStr(""),
                Headline::new(1, "Headline")
                    .and_property_drawer(PropertyDrawer::new(vec![NodeProperty::Key("test_name".to_string())]))
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
    fn test_property_drawer() {
        assert_eq!(
            property_drawer(CompleteStr(":PROPERTIES:\n:END:")).ok(),
            Some((
                CompleteStr(""),
                PropertyDrawer::empty()
            ))
        );
        assert_eq!(
            property_drawer(CompleteStr(":PROPERTIES:\n:test_name:\n:END:")).ok(),
            Some((
                CompleteStr(""),
                PropertyDrawer::new(vec![NodeProperty::Key("test_name".to_string())])
            ))
        );
    }

    #[test]
    fn test_node_property() {
        assert_eq!(
            node_property(CompleteStr(":some_name: some value")).ok(),
            Some((
                CompleteStr(""),
                NodeProperty::KeyValue("some_name".to_string(), "some value".to_string())
            ))
        );
        assert_eq!(
            node_property(CompleteStr(":some_name+: some value")).ok(),
            Some((
                CompleteStr(""),
                NodeProperty::KeyPlusValue("some_name".to_string(), "some value".to_string())
            ))
        );
        assert_eq!(
            node_property(CompleteStr(":some_name+:")).ok(),
            Some((
                CompleteStr(""),
                NodeProperty::KeyPlus("some_name".to_string())
            ))
        );
        assert_eq!(
            node_property(CompleteStr(":some_name:")).ok(),
            Some((
                CompleteStr(""),
                NodeProperty::Key("some_name".to_string())
            ))
        );
    }

    #[test]
    fn test_planning() {
        use chrono::NaiveDate;
        assert_eq!(
            planning(CompleteStr("DEADLINE: <2018-08-13>")).ok(),
            Some((
                CompleteStr(""),
                Planning::default().and_deadline(Timestamp::Active(TimestampData::new(
                    NaiveDate::from_ymd(2018, 08, 13)
                )))
            ))
        );
        assert_eq!(
            planning(CompleteStr("SCHEDULED: <2018-08-13>")).ok(),
            Some((
                CompleteStr(""),
                Planning::default().and_scheduled(Timestamp::Active(TimestampData::new(
                    NaiveDate::from_ymd(2018, 08, 13)
                )))
            ))
        );
        assert_eq!(
            planning(CompleteStr("CLOSED: [2018-08-13]")).ok(),
            Some((
                CompleteStr(""),
                Planning::default().and_closed(Timestamp::Inactive(TimestampData::new(
                    NaiveDate::from_ymd(2018, 08, 13)
                )))
            ))
        );
        assert_eq!(
            planning(CompleteStr("DEADLINE: <2018-08-13> CLOSED: [2018-08-13]")).ok(),
            Some((
                CompleteStr(""),
                Planning::default()
                    .and_closed(Timestamp::Inactive(TimestampData::new(
                        NaiveDate::from_ymd(2018, 08, 13)
                    ))).and_deadline(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(
                        2018, 08, 13
                    ))))
            ))
        );
    }

    #[test]
    fn test_level() {
        assert_eq!(level(CompleteStr("***")).ok(), Some((CompleteStr(""), 3)));
        assert_eq!(
            level(CompleteStr("***** Title here")).ok(),
            Some((CompleteStr(" Title here"), 5))
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
            Some((CompleteStr(""), Priority::A))
        );
        assert_eq!(
            priority(CompleteStr("[#Z] Headline")).ok(),
            Some((CompleteStr(" Headline"), Priority::Z))
        );
    }

    #[test]
    fn test_keyword() {
        assert_eq!(
            keyword(CompleteStr("TODO ")).ok(),
            Some((CompleteStr(" "), State::Todo("TODO".into())))
        );
        assert_eq!(
            keyword(CompleteStr("DONE Headline")).ok(),
            Some((CompleteStr(" Headline"), State::Done("DONE".into())))
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

    #[test]
    fn test_to_node_property() {
        assert_eq!(
            to_node_property("test_name", None),
            NodeProperty::Key("test_name".to_string())
        );
        assert_eq!(
            to_node_property("test_name+", None),
            NodeProperty::KeyPlus("test_name".to_string())
        );
        assert_eq!(
            to_node_property("test_name", Some("test_value")),
            NodeProperty::KeyValue("test_name".to_string(), "test_value".to_string())
        );
        assert_eq!(
            to_node_property("test_name+", Some("test_value")),
            NodeProperty::KeyPlusValue("test_name".to_string(), "test_value".to_string())
        );
    }

    #[test]
    fn test_to_keyword() {
        assert_eq!(
            to_keyword(CompleteStr("TODO")),
            Some(State::Todo("TODO".to_string()))
        );
        assert_eq!(
            to_keyword(CompleteStr("DONE")),
            Some(State::Done("DONE".to_string()))
        );
    }

    #[test]
    fn test_to_planning() {
        use chrono::NaiveDate;
        assert_eq!(
            to_planning((None, None, None)),
            Err(())
        );
        assert_eq!(
            to_planning((Some(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))), None, None)),
            Ok(Planning::default().and_deadline(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))
        );
        assert_eq!(
            to_planning((None, Some(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))), None)),
            Ok(Planning::default().and_scheduled(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))
        );
        assert_eq!(
            to_planning((None, None, Some(Timestamp::Inactive(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))),
            Ok(Planning::default().and_closed(Timestamp::Inactive(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))
        );
    }
}
