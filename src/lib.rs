#![feature(try_from)]

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate itertools;

#[macro_use]
pub mod macros;
#[macro_use]
mod enum_from_str;

mod parse;

use std::collections::HashMap;
use std::str::FromStr;

use chrono::prelude::*;
use chrono::Duration;

pub use parse::*;

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
    state: State,
    priority: Priority,
    //tags: Vec<String>,
    scheduled: Option<Timestamp>,
    deadline: Option<Timestamp>,
    closed: Option<Timestamp>,
    //timestamps: Vec<OrgTimestamp>,
    //properties: OrgProperties,
    content: OrgContent,
    //commented: bool,
    nodes: Vec<OrgNode>,
}

/// Represents the action that is taken when you mark a task with a repeater as `DONE`.
///
/// Contains the amount of time to use when repeating and the strategy
/// to use when applying the repeater (see [`RepeatStrategy`]).
#[derive(Debug, PartialEq, Eq)]
pub struct Repeater {
    amount: u32,
    unit: TimeUnit,
    strategy: RepeatStrategy,
}

impl Repeater {
    /// Constructs a new `Repeater` with the specified unit and amount and the
    /// `RepeatStrategy::AddOnce`.
    pub fn new(amount: u32, unit: TimeUnit) -> Self {
        Repeater {
            amount,
            unit,
            strategy: RepeatStrategy::AddOnce,
        }
    }

    /// Constructs a new `Repeater` with the specified unit, amount and strategy.
    pub fn with_strategy(amount: u32, unit: TimeUnit, strategy: RepeatStrategy) -> Self {
        Repeater {
            amount,
            unit,
            strategy,
        }
    }
}

/// The different repeat strategies that can be used.
///
/// * `AddOnce` will add the repeat duration to the date once.
/// * `AddUntilFuture` will add the repeat duration to the task date (at least once) until the
///   date is in the future.
/// * `AddToNow` will add the repeat duration to the current time.
#[derive(Debug, PartialEq, Eq)]
pub enum RepeatStrategy {
    AddOnce,
    AddUntilFuture,
    AddToNow,
}

/// Represents the amount of time to warn the user before a task is due.
#[derive(Debug, PartialEq, Eq)]
pub struct WarningPeriod {
    amount: u32,
    unit: TimeUnit,
}

impl WarningPeriod {
    /// Constructs a new `WarningPeriod` with the specified unit and amount.
    pub fn new(amount: u32, unit: TimeUnit) -> Self {
        WarningPeriod { amount, unit }
    }
}

/// Helper trait implemented on `u32` to easily convert to a `WarningPeriod`.
trait AsWarningPeriod {
    /// Convert self to a `WarningPeriod` wit unit `TimeUnit::Year`.
    fn year(self) -> WarningPeriod;
    /// Convert self to a `WarningPeriod` wit unit `TimeUnit::Month`.
    fn month(self) -> WarningPeriod;
    /// Convert self to a `WarningPeriod` wit unit `TimeUnit::Week`.
    fn week(self) -> WarningPeriod;
    /// Convert self to a `WarningPeriod` wit unit `TimeUnit::Day`.
    fn day(self) -> WarningPeriod;
    /// Convert self to a `WarningPeriod` wit unit `TimeUnit::Hour`.
    fn hour(self) -> WarningPeriod;
}

impl AsWarningPeriod for u32 {
    fn year(self) -> WarningPeriod {
        WarningPeriod::new(self, TimeUnit::Year)
    }
    fn month(self) -> WarningPeriod {
        WarningPeriod::new(self, TimeUnit::Month)
    }
    fn week(self) -> WarningPeriod {
        WarningPeriod::new(self, TimeUnit::Week)
    }
    fn day(self) -> WarningPeriod {
        WarningPeriod::new(self, TimeUnit::Day)
    }
    fn hour(self) -> WarningPeriod {
        WarningPeriod::new(self, TimeUnit::Hour)
    }
}

/// Represents the unit of time used for `Repeater` and `WarningPeriod`.
#[derive(Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
}

/// Represents a date in an org file. See [https://orgmode.org/manual/Timestamps.html].
/// TODO convert to struct and add warning period to all variants
#[derive(Debug, PartialEq, Eq)]
pub enum Timestamp {
    InactiveDate(NaiveDate),
    InactiveDatetime(NaiveDateTime),
    ActiveDate(NaiveDate),
    ActiveDatetime(NaiveDateTime),
    TimeRange {
        date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
    },
    DateRange(NaiveDate, NaiveDate),
    DatetimeRange(NaiveDateTime, NaiveDateTime),
    RepeatingDate(NaiveDate, Repeater, Option<WarningPeriod>),
    RepeatingDatetime(NaiveDateTime, Repeater, Option<WarningPeriod>),
}

impl Timestamp {
    /// Returns `true` if the org timestamp is active.
    ///
    /// This is the case if it is not one of [`InactiveDate`] or [`InactiveDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    ///
    /// let x = Timestamp::ActiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), true);
    ///
    /// let x = Timestamp::InactiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), false);
    /// ```
    ///
    /// [`InactiveDate`]: #variant.InactiveDate
    /// [`InactiveDateTime`]: #variant.InactiveDateTime
    pub fn is_active(&self) -> bool {
        match self {
            Timestamp::InactiveDate(_) => false,
            Timestamp::InactiveDatetime(_) => false,
            _ => true,
        }
    }

    /// Returns `true` if the org timestamp is inactive.
    ///
    /// This is the case if it is eighter [`InactiveDate`] or [`InactiveDateTime`].
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    ///
    /// let x = Timestamp::ActiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), true);
    ///
    /// let x = Timestamp::InactiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), false);
    /// ```
    ///
    /// [`InactiveDate`]: #variant.InactiveDate
    /// [`InactiveDateTime`]: #variant.InactiveDateTime
    pub fn is_inactive(&self) -> bool {
        !self.is_active()
    }
}

/// Parses the second line of a org node. This line can contain any of closed, scheduled and deadline
/// date or none of them.
///
/// The dates are preceded by their respective keyword (`CLOSED`, `DEADLINE`, `SCHEDULED`) followed
/// by a `:`, a space and the actual date. The date of closed is inactive and therefore surrounded by square brackets (`[`, `]`). The date of scheduled and deadline are plain timestamps or timestamps with a repeat interval and therefore surrounded by angle brackets (`<`, `>`).
//fn parse_special_node_timestamps(line: &str) -> SpecialNodeTimestamps {
//    lazy_static! {
//        static ref RE_OUTER: Regex =
//            Regex::new(r"^\s*((?:DEADLINE|SCHEDULED|CLOSED):\s+(?:\[.+\]|<.+>)\s*)+").unwrap();
//        static ref RE_ITEM: Regex =
//            Regex::new(r"(?P<kind>DEADLINE|SCHEDULED|CLOSED):\s+(?P<ts>\[.+\]|<.+>)").unwrap();
//    }
//
//    RE_OUTER
//        .find(line)
//        .map(|truncated| {
//            RE_ITEM
//                .captures_iter(truncated.as_str())
//                .map(|cap| {
//                    (
//                        cap.name("kind").map(|m| m.as_str()),
//                        cap.name("ts").map(|m| m.as_str()),
//                    )
//                })
//                .map(|x| x.into())
//                .fold(SpecialNodeTimestamps::default(), |acc, x| acc.and(x))
//        })
//        .unwrap_or_default()
//}

/// The state of a [`OrgNode`]. Can be eighter `Todo` or `Done`. The enum variants accept an
/// additional string because the actual keyword signaling the state of the `OrgNode` can be
/// anything.
///
/// `TODO` and `NEXT` will be parsed as `State::Todo` and `DONE` will be parsed as `State::Done`.
/// An empty string will be parsed as `State::None`.
#[derive(Debug, PartialEq, Eq)]
pub enum State {
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
    fn test_a_to_z_is_parseable_to_priority() {
        use std::char;

        for i in 'A' as u32..('Z' as u32 + 1) {
            let prio = Priority::from_str(&char::from_u32(i).unwrap().to_string());
            assert!(prio.is_ok());
        }
    }

    #[test]
    fn test_as_warning_period() {
        use TimeUnit::*;
        assert_eq!(2018.year(), WarningPeriod::new(2018, Year));
        assert_eq!(2.month(), WarningPeriod::new(2, Month));
        assert_eq!(6.week(), WarningPeriod::new(6, Week));
        assert_eq!(8.day(), WarningPeriod::new(8, Day));
        assert_eq!(3.hour(), WarningPeriod::new(3, Hour));
    }

}
