use std::convert::TryInto;

use {Headline, NodeProperty, PropertyDrawer, Planning, Timestamp, Section, Priority, State};
use parse::{OrgInput, OrgResult, timestamp, affiliated_keywords};

/// Parses the stars at the beginning of the line to their count.
fn level(i: OrgInput) -> OrgResult<u8> {
    to_failure!(i, map_res!(
        take_while1!(|c| c == '*'),
        |s: OrgInput| (*s).len().try_into()
    ))
}

/// Parses the keyword at the beginning of the headline (after the stars).
fn keyword(i: OrgInput) -> OrgResult<State> {
    to_failure!(i, map_opt!(
        take_until!(" "),
        to_keyword
    ))
}

/// Converts the string to a keyword.
fn to_keyword(i: OrgInput) -> Option<State> {
    // TODO make this more dynamic
    match *i {
        "TODO" => Some(State::Todo(String::from(*i))),
        "DONE" => Some(State::Done(String::from(*i))),
        _ => None,
    }
}

/// Parses the priority of the headline.
fn priority(i: OrgInput) -> OrgResult<Priority> {
    to_failure!(i, map_res!(
        to_failure!(do_parse!(
            tag!("[#") >>
            prio: take!(1) >>
            tag!("]") >>
            (prio)
        )),
        |i: OrgInput| (*i).parse()
    ))
}

/// Check if the given char is a valid tag char (excluding the seperators `:`).
///
/// Valid tag chars are alpha-numeric characters, underscores, at signs, hash signs and percent signs.
fn is_tags_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '@' || c == '#' || c == '%'
}

/// Finds the start of the tags in a headline. Tags has to be the last thing in the headline,
/// optionally followed by whitespace.
///
/// Returns `None` if there are no tags.
fn find_tags_start(input: &OrgInput) -> Option<usize> {
    use nom::{FindSubstring, InputTake, InputIter};

    enum MyState {
        InTags,
        OnTagsBorder,
        OutsideTags(bool),
    }

    if let Some(start) = input.find_substring(":") {
        let mut index = 0;
        let mut state = MyState::InTags;
        let (start_str, _) = input.take_split(start);

        for (i, c) in start_str.iter_indices() {
            match state {
                MyState::InTags if c == ':' => {
                    // reached posssible end of tags
                    state = MyState::OnTagsBorder;
                },
                MyState::InTags if is_tags_char(c) => {},
                MyState::InTags => {
                    // assumed we were in tag but really weren't
                    state = MyState::OutsideTags(c.is_whitespace());
                },
                MyState::OnTagsBorder if is_tags_char(c) => {
                    state = MyState::InTags;
                },
                MyState::OnTagsBorder if c == ':' => {},
                MyState::OnTagsBorder => {
                    state = MyState::OutsideTags(c.is_whitespace());
                },
                MyState::OutsideTags(_) if c == ':' => {
                    index = i;
                    state = MyState::InTags;
                },
                MyState::OutsideTags(_) => {
                    state = MyState::OutsideTags(c.is_whitespace());
                },
            }
        }

        match state {
            MyState::OutsideTags(false) => None,
            _ => Some(start + index),
        }
    } else {
        None
    }
}

/// Parser that returns the title as a result.
///
/// This is manually implemented instead of with macros because it was easier.
fn take_title(input: OrgInput) -> OrgResult<OrgInput> {
    use nom::{InputLength, FindSubstring, InputTake};

    let newline_at = input.find_substring("\n").unwrap_or(input.input_len());
    let (rest, title_and_tags) = input.take_split(newline_at);

    match find_tags_start(&title_and_tags) {
        Some(index) => {
            let (_, title_with_whitespace) = input.take_split(index);

            match title_with_whitespace.rfind(|c: char| !c.is_whitespace()) {
                Some(last_non_ws) => Ok(input.take_split(last_non_ws + 1)),
                None => Ok((rest, title_and_tags))
            }

        },
        None => Ok((rest, title_and_tags))
    }
}

/// Parses the title of a headline.
fn title(i: OrgInput) -> OrgResult<String> {
    to_failure!(i, map!(
        take_title,
        |i: OrgInput| String::from(*i)
    ))
}

/// Parses the tags of a headline.
///
/// The tags are made of words containing any alpha-numeric character, underscore,
/// at sign, hash sign or percent sign, and separated with colons.
///
/// E.g. `:tag:a2%:` which is two tags `tag` and `a2%`.
fn tags(i: OrgInput) -> OrgResult<Vec<String>> {
    to_failure!(i, delimited!(
        tag!(":"),
        separated_list_complete!(
            tag!(":"),
            map!(
                take_until!(":"),
                |i: OrgInput| String::from(*i)
            )
        ),
        tag!(":")
    ))
}

/// Parses a section.
///
/// Currently just takes all input until a new headline begins.
pub fn section(i: OrgInput) -> OrgResult<Section> {
    to_failure!(i, map!(
        // TODO maybe matching \n* is not the best,
        preceded!(
            not!(tag!("*")),
            take_until_or_eof!("\n*")
        ),
        |i: OrgInput| Section::new(*i)
    ))
}

/// Parses a planning line. (optional line directly under the headline)
fn planning(i: OrgInput) -> OrgResult<Planning> {
    map_opt!(
        i,
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
}

/// Converts a deadline, scheduled and closed timestamp (all optional) to a [`Planning`] object.
fn to_planning(
    (deadline, scheduled, closed): (Option<Timestamp>, Option<Timestamp>, Option<Timestamp>),
) -> Option<Planning> {
    if deadline.is_none() && scheduled.is_none() && closed.is_none() {
        None
    } else {
        Some(Planning::default()
            .and_opt_deadline(deadline)
            .and_opt_scheduled(scheduled)
            .and_opt_closed(closed))
    }
}

/// Parses a property drawer with node properties.
///
/// TODO (for later) make this recognize an indented property drawer
fn property_drawer(i: OrgInput) -> OrgResult<PropertyDrawer> {
    do_parse!(i,
        to_failure!(tag!(":PROPERTIES:\n")) >>
        list: opt!(separated_list!(to_failure!(tag!("\n")), node_property)) >>
        to_failure!(opt!(tag!("\n"))) >>
        to_failure!(tag!(":END:")) >>
        (PropertyDrawer::new(list.unwrap_or_default()))
    )
}

/// Parses a single node property of a property drawer.
///
/// Can be of the following formats:
///
/// - `:NAME: VALUE`
/// - `:NAME+: VALUE`
/// - `:NAME:`
/// - `:NAME+:`
///
/// **Note:** `NAME` can't be `END`.
fn node_property(i: OrgInput) -> OrgResult<NodeProperty> {
    to_failure!(i, do_parse!(
        name: verify!(
            delimited!(tag!(":"), take_while!(|c| c != ':'), tag!(":")),
            |name: OrgInput| *name != "END"
        ) >>
        value: opt!(preceded!(tag!(" "), take_while!(|c| c != '\n'))) >>
        (to_node_property(*name, value.map(|v| *v)))
    ))
}

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

/// Parses a complete headline.
///
/// Has the format:
///
/// ```text
/// AFFILIATED_KEYWORDS
/// STARS KEYWORD PRIORITY TITLE TAGS
/// PLANNING
/// PROPERTY_DRAWER
/// SECTION
/// ```
///
/// Where `KEYWORD`, `PRIORITY`, `TAGS`, `PLANNING`, `PROPERTY_DRAWER` and `SECTION` are optional.
///
/// `TAGS` is not yet implemented.
///
/// For the formats of the items see:
///
/// - `AFFILIATED_KEYWORDS`: [`affiliated_keywords`]
/// - `STARS`: [`level`]
/// - `KEYWORD`: [`keyword`]
/// - `PRIORITY`: [`priority`]
/// - `TITLE`: [`title`]
/// - `TAGS`: [`tags`]
/// - `PLANNING`: [`planning`]
/// - `PROPERTY_DRAWER`: [`property_drawer`]
/// - `SECTION`: [`section`]
pub fn headline(i: OrgInput) -> OrgResult<Headline> {
    to_failure!(i, do_parse!(
        affiliated_keywords: opt!(terminated!(
            affiliated_keywords,
            to_failure!(tag!("\n"))
        )) >>
        level: level >>
        keyword: opt!(preceded!(
            to_failure!(tag!(" ")),
            keyword
        )) >>
        priority: opt!(preceded!(
            to_failure!(tag!(" ")),
            priority
        )) >>
        to_failure!(tag!(" ")) >>
        title: title >>
        tags: opt!(preceded!(
            to_failure!(tag!(" ")),
            tags
        )) >>
        planning: opt!(preceded!(
            to_failure!(tag!("\n")),
            planning
        )) >>
        property_drawer: opt!(preceded!(
            to_failure!(tag!("\n")),
            property_drawer
        )) >>
        section: opt!(preceded!(
            to_failure!(tag!("\n")),
            section
        )) >>
        //to_failure!(opt!(tag!("\n"))) >>
        (
            Headline::new(level, title)
                .and_affiliated_keywords(affiliated_keywords.unwrap_or_default())
                .and_opt_keyword(keyword)
                .and_opt_priority(priority)
                .and_opt_tags(tags)
                .and_planning(planning.unwrap_or_default())
                .and_property_drawer(property_drawer.unwrap_or_default())
                .and_opt_section(section.filter(|section| !section.is_empty()))
        )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use {TimestampData, AffiliatedKeyword, AffiliatedKeywordKind, AffiliatedKeywordValue};
    use nom::types::CompleteStr;

    #[test]
    fn test_headline_with_affiliated_keywords() {
        assert_eq!(
            headline(CompleteStr("#+CAPTION: some caption\n* Headline")).ok(),
            Some((
                CompleteStr(""),
                Headline::new(1, "Headline")
                    .and_affiliated_keywords(vec![
                        AffiliatedKeyword::new(
                            AffiliatedKeywordKind::Caption(None),
                            AffiliatedKeywordValue::new("some caption")
                        )
                    ])
            ))
        );
        assert_eq!(
            headline(CompleteStr("#+CAPTION: some caption\n#+ATTR_backend: value\n* Headline")).ok(),
            Some((
                CompleteStr(""),
                Headline::new(1, "Headline")
                    .and_affiliated_keywords(vec![
                        AffiliatedKeyword::new(
                            AffiliatedKeywordKind::Caption(None),
                            AffiliatedKeywordValue::new("some caption")
                        ),
                        AffiliatedKeyword::new(
                            AffiliatedKeywordKind::Attr("backend".to_string()),
                            AffiliatedKeywordValue::new("value")
                        )
                    ])
            ))
        );
    }

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
        assert_eq!(
            headline(CompleteStr(
                "* TODO [#A] Headline with keyword and priority :tag1:tag2:"
            )).ok(),
            Some((
                CompleteStr(""),
                Headline::new(1, "Headline with keyword and priority")
                    .and_priority(Priority::A)
                    .and_keyword(State::Todo("TODO".into()))
                    .and_tags(vec!["tag1".into(), "tag2".into()])
            ))
        );
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
    fn test_find_tags_start() {
        assert_eq!(
            find_tags_start(&CompleteStr("some text")),
            None
        );
        assert_eq!(
            find_tags_start(&CompleteStr("some text :tags:yay:")),
            Some(10)
        );
        assert_eq!(
            find_tags_start(&CompleteStr("some text :tags:yay:   ")),
            Some(10)
        );
        assert_eq!(
            find_tags_start(&CompleteStr("some text :tags:yay: more text")),
            None
        );
        assert_eq!(
            find_tags_start(&CompleteStr("some text :not:tags: more :actual:tags:")),
            Some(26)
        );
        assert_eq!(
            find_tags_start(&CompleteStr("some text :not:tags: more :actual:tags:   ")),
            Some(26)
        );
        assert_eq!(
            find_tags_start(&CompleteStr("some text :not:tags: more :still:no:actual:tags: more text")),
            None
        );
    }

    #[test]
    fn test_take_title() {
        assert_eq!(
            take_title(CompleteStr("some title")).ok(),
            Some((CompleteStr(""), CompleteStr("some title")))
        );
        assert_eq!(
            take_title(CompleteStr("some title\n")).ok(),
            Some((CompleteStr("\n"), CompleteStr("some title")))
        );
        assert_eq!(
            take_title(CompleteStr("some title :some:tags:")).ok(),
            Some((CompleteStr(" :some:tags:"), CompleteStr("some title")))
        );
        assert_eq!(
            take_title(CompleteStr("some title :some:tags: more text")).ok(),
            Some((CompleteStr(""), CompleteStr("some title :some:tags: more text")))
        );
        assert_eq!(
            take_title(CompleteStr("some title :some:tags: more text :tags:")).ok(),
            Some((CompleteStr(" :tags:"), CompleteStr("some title :some:tags: more text")))
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
            None
        );
        assert_eq!(
            to_planning((Some(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))), None, None)),
            Some(Planning::default().and_deadline(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))
        );
        assert_eq!(
            to_planning((None, Some(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))), None)),
            Some(Planning::default().and_scheduled(Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))
        );
        assert_eq!(
            to_planning((None, None, Some(Timestamp::Inactive(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))),
            Some(Planning::default().and_closed(Timestamp::Inactive(TimestampData::new(NaiveDate::from_ymd(2018, 08, 25)))))
        );
    }
}
