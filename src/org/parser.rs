use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use regex::Captures;
use regex::Regex;

use org::*;

/// Error returned by [`parse_node`]. The variants should be self expanatory.
#[derive(Debug, PartialEq, Eq)]
pub enum OrgNodeParseError {
    ExpectedNewHeadline,
}

/// Parse a node (and recursively all of its sub-nodes) from the given string.
///
/// Returns an error if it doesn't find a correctly formatted headline at the start of the
/// given string. Stops when the string ends or if it finds another headline with same level.
///
/// *org nodes* look something like this:
///
/// ```org
/// * [#A] Some Headline :tag1: :tag2:
/// SCHEDULED: <2018-04-26 Thu 14:00 .+1w>
///
/// Some text here is optional. This section can contains anything, including tables, lists, etc.
///
/// ** TODO Read something :tag2:
/// DEADLINE: <2018-04-30 Mon 12:00>
///
/// ** DONE Draw something :tag1:
/// CLOSED: [2018-04-24 Tue 09:50] DEADLINE: <2018-04-24 Tue 10:00>
/// *** DONE Draw the head
/// CLOSED: [2018-04-24 Tue 08:50]
/// *** DONE Draw the body
/// CLOSED: [2018-04-24 Tue 09:10]
/// *** DONE Draw the clothes
/// CLOSED: [2018-04-24 Tue 09:40]
/// ```
pub fn parse_node(text: &str) -> Result<OrgNode, OrgNodeParseError> {
    let mut lines = text.lines();

    let first_line = lines.next();
    let second_line = lines.next();

    let level = count_prefix_chars(text, '*');

    if (level == 0) {
        return Err(OrgNodeParseError::ExpectedNewHeadline);
    }

    let (closed, scheduled, deadline) = parse_special_node_timestamps(second_line.unwrap());

    Ok(OrgNode::default())
}

fn count_prefix_chars(s: &str, needle: char) -> usize {
    s.chars().take_while(|c| c == &needle).count()
}

/// parses the second line of a org node. this line can contain any of closed, scheduled and deadline
/// date or none of them.
///
/// the dates are preceded by their respective keyword (`closed`, `deadline`, `scheduled`) followed
/// by a `:`, a space and the actual date. the date of closed is inactive and therefore surrounded by square brackets (`[`, `]`). the date of scheduled and deadline are plain timestamps or timestamps with a repeat interval and therefore surrounded by angle brackets (`<`, `>`).
fn parse_special_node_timestamps(
    line: &str,
) -> (
    Option<OrgTimestamp>,
    Option<OrgTimestamp>,
    Option<OrgTimestamp>,
) {
    return (None, None, None);
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrgTimestampParseError {
    ParseError,
}

pub fn parse_timestamp(s: &str) -> Result<OrgTimestamp, OrgTimestampParseError> {
    let trimmed = s.trim();

    let date_regex = Regex::new(r"(?P<prefix>[<\[])(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})( (?P<weekday>[A-Z][a-z]{2}))?( (?P<rest>.*))?(?P<suffix>[>\]])").unwrap();
    let time_regex = Regex::new(r"(?P<hours>\d{2}):(?P<minutes>\d{2})").unwrap();

    let caps = date_regex.captures(trimmed);

    let date = match &caps {
        Some(caps) => get_date_from_captures(&caps).ok_or(OrgTimestampParseError::ParseError),
        None => Err(OrgTimestampParseError::ParseError),
    }?;

    let time_caps = caps.and_then(|caps| time_regex.captures(caps.name("rest")?.as_str()));

    let time = time_caps.and_then(|caps| get_time_from_captures(&caps));

    Ok(match time {
        Some(time) => OrgTimestamp::ActiveDateTime(date.and_time(time)),
        None => OrgTimestamp::ActiveDate(date),
    })
}

fn get_date_from_captures<'t>(caps: &Captures<'t>) -> Option<NaiveDate> {
    //println!("Date: {:#?}", caps);
    let year = caps.name("year")?.as_str().parse().ok()?;
    let month = caps.name("month")?.as_str().parse().ok()?;
    let day = caps.name("day")?.as_str().parse().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
}

fn get_time_from_captures<'t>(caps: &Captures<'t>) -> Option<NaiveTime> {
    //println!("Time: {:#?}", caps);
    let hours = caps.name("hours")?.as_str().parse().ok()?;
    let minutes = caps.name("minutes")?.as_str().parse().ok()?;

    NaiveTime::from_hms_opt(hours, minutes, 0)
}

fn surrounded_with(s: &str, start: char, end: char) -> bool {
    s.starts_with(start) && s.ends_with(end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_prefix_chars() {
        assert_eq!(count_prefix_chars("* abc", '*'), 1);
        assert_eq!(count_prefix_chars("*** abc *", '*'), 3);
        assert_eq!(count_prefix_chars("****** abc ** asd *", '*'), 6);
        assert_eq!(count_prefix_chars("* abc ** a", '*'), 1);
        assert_eq!(count_prefix_chars("abs * abc", '*'), 0);
    }

    /// Helper function to generate a NaiveDateTime easily
    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> NaiveDateTime {
        NaiveDate::from_ymd(year, month, day).and_time(NaiveTime::from_hms(hour, min, sec))
    }

    #[test]
    #[ignore]
    fn test_parse_special_node_timestamps() {
        assert_eq!(
            parse_special_node_timestamps("DEADLINE: <2018-02-19 Mon 14:24>"),
            (
                Some(OrgTimestamp::ActiveDateTime(naive_date_time(
                    2018, 2, 19, 14, 24, 0
                ))),
                None,
                None
            )
        );
    }

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(
            parse_timestamp("<2018-06-22 Fri>"),
            Ok(OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 6, 22)))
        );
        assert_eq!(
            parse_timestamp("<2018-06-22>"),
            Ok(OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 6, 22)))
        );
        assert_eq!(
            parse_timestamp("<2018-06-22 Fri 14:00>"),
            Ok(OrgTimestamp::ActiveDateTime(naive_date_time(
                2018, 6, 22, 14, 0, 0
            )))
        );
        assert_eq!(
            parse_timestamp("<2018-06-22 14:00>"),
            Ok(OrgTimestamp::ActiveDateTime(naive_date_time(
                2018, 6, 22, 14, 0, 0
            )))
        );
    }
}
