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

/// Parses the second line of a org node. This line can contain any of closed, scheduled and deadline
/// date or none of them.
///
/// The dates are preceded by their respective keyword (`CLOSED`, `DEADLINE`, `SCHEDULED`) followed
/// by a `:`, a space and the actual date. The date of closed is inactive and therefore surrounded by square brackets (`[`, `]`). The date of scheduled and deadline are plain timestamps or timestamps with a repeat interval and therefore surrounded by angle brackets (`<`, `>`).
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
    Err(OrgTimestampParseError::ParseError)
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
    #[ignore]
    fn test_parse_timestamp() {
        assert_eq!(
            parse_timestamp("<2018-06-22 Fri 14:00>"),
            Ok(OrgTimestamp::ActiveDateTime(naive_date_time(
                2018, 6, 22, 14, 0, 0
            )))
        );
    }
}
