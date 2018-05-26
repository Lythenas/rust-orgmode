mod helpers;
mod timestamp;
#[macro_use]
mod enum_from_str;

use std::collections::HashMap;
use std::str::FromStr;

use chrono::prelude::*;
use chrono::Duration;
use regex::Captures;
use regex::Regex;

pub use org::timestamp::*;

/// Represents a org file.
#[derive(Debug, PartialEq, Eq)]
pub struct OrgFile {
    preface: String,
    properties: HashMap<String, String>,
    nodes: Vec<OrgNode>,
}

impl FromStr for OrgFile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!();
    }
}

/// Represents one *node* in the org file. A node is a headline (a line starting with one or more `*`).
///
/// This node can contain many more nodes that are sub-headlines of this one. (It is a tree or
/// sub-tree).
#[derive(Debug, PartialEq, Eq)]
pub struct OrgNode {
    level: u8,
    title: String,
    state: OrgState,
    priority: Priority,
    //tags: Vec<String>,
    scheduled: Option<OrgTimestamp>,
    deadline: Option<OrgTimestamp>,
    closed: Option<OrgTimestamp>,
    //timestamps: Vec<OrgTimestamp>,
    //properties: OrgProperties,
    content: OrgContent,
    //commented: bool,
    nodes: Vec<OrgNode>,
}

/// Helper struct returned by [`parse_special_node_timestamps`].
#[derive(Debug, PartialEq, Eq, Default)]
struct SpecialNodeTimestamps {
    deadline: Option<OrgTimestamp>,
    scheduled: Option<OrgTimestamp>,
    closed: Option<OrgTimestamp>,
}

impl SpecialNodeTimestamps {
    fn and(self, other: Self) -> Self {
        SpecialNodeTimestamps {
            deadline: self.deadline.or(other.deadline),
            scheduled: self.scheduled.or(other.scheduled),
            closed: self.closed.or(other.closed),
        }
    }
}

impl<'a, 'b> From<(Option<&'a str>, Option<&'b str>)> for SpecialNodeTimestamps {
    fn from((kind, timestamp): (Option<&str>, Option<&str>)) -> Self {
        let map_true = |x| if x { Some(()) } else { None };
        let map_to_timestamp = |_| timestamp.and_then(|t| t.parse().ok());

        let deadline = kind
            .map(|x| x == "DEADLINE")
            .and_then(map_true)
            .and_then(map_to_timestamp);
        let scheduled = kind
            .map(|x| x == "SCHEDULED")
            .and_then(map_true)
            .and_then(map_to_timestamp);
        let closed = kind
            .map(|x| x == "CLOSED")
            .and_then(map_true)
            .and_then(map_to_timestamp);

        SpecialNodeTimestamps {
            deadline,
            scheduled,
            closed,
        }
    }
}

/// Parses the second line of a org node. This line can contain any of closed, scheduled and deadline
/// date or none of them.
///
/// The dates are preceded by their respective keyword (`CLOSED`, `DEADLINE`, `SCHEDULED`) followed
/// by a `:`, a space and the actual date. The date of closed is inactive and therefore surrounded by square brackets (`[`, `]`). The date of scheduled and deadline are plain timestamps or timestamps with a repeat interval and therefore surrounded by angle brackets (`<`, `>`).
fn parse_special_node_timestamps(line: &str) -> SpecialNodeTimestamps {
    lazy_static! {
        static ref RE_OUTER: Regex =
            Regex::new(r"^\s*((?:DEADLINE|SCHEDULED|CLOSED):\s+(?:\[.+\]|<.+>)\s*)+").unwrap();
        static ref RE_ITEM: Regex =
            Regex::new(r"(?P<kind>DEADLINE|SCHEDULED|CLOSED):\s+(?P<ts>\[.+\]|<.+>)").unwrap();
    }

    RE_OUTER
        .find(line)
        .map(|truncated| {
            RE_ITEM
                .captures_iter(truncated.as_str())
                .map(|cap| {
                    (
                        cap.name("kind").map(|m| m.as_str()),
                        cap.name("ts").map(|m| m.as_str()),
                    )
                })
                .map(|x| x.into())
                .fold(SpecialNodeTimestamps::default(), |acc, x| acc.and(x))
        })
        .unwrap_or_default()
}

/// Contains all the string accepted as [`OrgState::Todo`].
static ORG_TODO_STATES: [&'static str; 2] = ["TODO", "NEXT"];

/// Contains all the string accepted as [`OrgState::Done`].
static ORG_DONE_STATES: [&'static str; 1] = ["DONE"];

/// The state of a [`OrgNode`]. Can be eighter `Todo` or `Done`. The enum variants accept an
/// additional string because the actual keyword signaling the state of the `OrgNode` can be
/// anything.
///
/// Currently only keywords specified in [`ORG_TODO_STATES`] are parsed as `Todo`. All other
/// keywords is parsed as `Done`. No keyword present a.k.a an empty string will be parsed as
/// `None`.
#[derive(Debug, PartialEq, Eq)]
pub enum OrgState {
    Todo(String),
    Done(String),
    None,
}

pub type OrgProperties = HashMap<String, String>;

/// Represents the content (section) for one headline.
///
/// TODO make this more detailed than just a string
#[derive(Debug, PartialEq, Eq, Default)]
pub struct OrgContent {
    value: String,
}

enum_from_str!(
    Priority => A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_to_z_is_parsable_to_priority() {
        use std::char;

        for i in 'A' as u32..('Z' as u32 + 1) {
            let prio = Priority::from_str(&char::from_u32(i).unwrap().to_string());
            assert!(prio.is_ok());
        }
    }

    #[test]
    fn active_org_timestamp() {
        let ts = OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 1, 1));
        assert_eq!(ts.is_inactive(), true);
        assert_eq!(ts.is_active(), false);

        let ts2 = OrgTimestamp::InactiveDateTime(
            NaiveDate::from_ymd(2018, 1, 1).and_time(NaiveTime::from_hms(0, 0, 0)),
        );
        assert_eq!(ts2.is_inactive(), true);
        assert_eq!(ts2.is_active(), false);

        let ts3 = OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 1, 1));
        assert_eq!(ts3.is_active(), true);
        assert_eq!(ts3.is_inactive(), false);
    }

    #[test]
    fn test_parse_special_node_timestamps() {
        assert_eq!(
            parse_special_node_timestamps("DEADLINE: <2018-02-19 Mon 14:24>"),
            SpecialNodeTimestamps {
                deadline: Some(OrgTimestamp::ActiveDateTime(
                    NaiveDate::from_ymd(2018, 2, 19).and_hms(14, 24, 0)
                )),
                scheduled: None,
                closed: None
            }
        );
        assert_eq!(
            parse_special_node_timestamps(
                "CLOSED: [2018-02-11 15:33] DEADLINE: <2018-02-19 Mon 14:24>"
            ),
            SpecialNodeTimestamps {
                deadline: Some(OrgTimestamp::ActiveDateTime(
                    NaiveDate::from_ymd(2018, 2, 19).and_hms(14, 24, 0)
                )),
                scheduled: None,
                closed: Some(OrgTimestamp::InactiveDateTime(
                    NaiveDate::from_ymd(2018, 2, 11).and_hms(15, 33, 0)
                ))
            }
        );
        assert_eq!(
            parse_special_node_timestamps("CLOSED: [2018-02-11] SCHEDULED: <2018-02-11>"),
            SpecialNodeTimestamps {
                deadline: None,
                scheduled: Some(OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 2, 11))),
                closed: Some(OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 2, 11)))
            }
        );
        assert_eq!(
            parse_special_node_timestamps("Some text that is not a recognized timestamp."),
            SpecialNodeTimestamps {
                deadline: None,
                scheduled: None,
                closed: None,
            }
        );
        assert_eq!(
            parse_special_node_timestamps("Text before timestamps CLOSED: [2018-05-18] and after"),
            SpecialNodeTimestamps {
                deadline: None,
                scheduled: None,
                closed: None
            }
        );
    }

}
