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

pub use parse::*;

/// Represents an org file.
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
    timestamps: Vec<Timestamp>,
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
    period: TimePeriod,
    strategy: RepeatStrategy,
}

impl Repeater {
    /// Constructs a new `Repeater` with the specified time period and repeat strategy.
    pub fn new(period: TimePeriod, strategy: RepeatStrategy) -> Self {
        Repeater { period, strategy }
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

/// Represents a amount of time.
///
/// Used e.g. as the warning period and in repeater.
#[derive(Debug, PartialEq, Eq)]
pub struct TimePeriod {
    amount: u32,
    unit: TimeUnit,
}

impl TimePeriod {
    /// Constructs a new `TimePeriod` with the specified unit and amount.
    pub fn new(amount: u32, unit: TimeUnit) -> Self {
        Self { amount, unit }
    }
}

/// Convenience trait implemented on `u32` to easily convert to a `TimePeriod`.
pub trait AsTimePeriod {
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Year`.
    fn year(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Month`.
    fn month(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Week`.
    fn week(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Day`.
    fn day(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Hour`.
    fn hour(self) -> TimePeriod;
}

impl AsTimePeriod for u32 {
    fn year(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Year)
    }
    fn month(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Month)
    }
    fn week(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Week)
    }
    fn day(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Day)
    }
    fn hour(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Hour)
    }
}

/// Represents the unit of time used for `Repeater` and `TimePeriod`.
#[derive(Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
}

/// Represents a date in an org file. See [https://orgmode.org/manual/Timestamps.html].
#[derive(Debug, PartialEq, Eq)]
pub struct Timestamp {
    kind: TimestampKind,
    warning_period: Option<TimePeriod>,
}

/// Part of a [`Timestamp`].
#[derive(Debug, PartialEq, Eq)]
pub enum TimestampKind {
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
    RepeatingDate(NaiveDate, Repeater),
    RepeatingDatetime(NaiveDateTime, Repeater),
}

impl TimestampKind {
    /// Returns true if the timestamp kind is active.
    ///
    /// This is the case if it is not [`InactiveDate`] or [`InactiveDateTime`].
    pub fn is_active(&self) -> bool {
        match self {
            TimestampKind::InactiveDate(_) => false,
            TimestampKind::InactiveDatetime(_) => false,
            _ => true,
        }
    }
}

impl Timestamp {
    /// Creates a new Timestamp from the given [`TimestampKind`] with no warning period.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    /// # use orgmode::TimestampKind;
    /// # use orgmode::AsTimePeriod;
    /// #
    /// let ts = Timestamp::new(
    ///     TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22))
    /// );
    /// assert_eq!(ts.get_kind(), &TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    /// assert_eq!(ts.get_warning_period(), &None);
    /// ```
    ///
    /// See: [`TimestampKind`]
    pub fn new(kind: TimestampKind) -> Self {
        Timestamp {
            kind,
            warning_period: None,
        }
    }

    /// Creates a new Timestamp with the given [`TimestampKind`] and warning period.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    /// # use orgmode::TimestampKind;
    /// # use orgmode::AsTimePeriod;
    /// #
    /// let ts = Timestamp::with_warning_period(
    ///     TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)), 1.day()
    /// );
    /// assert_eq!(ts.get_kind(), &TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    /// assert_eq!(ts.get_warning_period(), &Some(1.day()));
    /// ```
    pub fn with_warning_period(kind: TimestampKind, warning_period: TimePeriod) -> Self {
        Self::with_warning_period_opt(kind, Some(warning_period))
    }

    /// Creates a new Timestamp with the given [`TimestampKind`] and optional warning period.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    /// # use orgmode::TimestampKind;
    /// # use orgmode::AsTimePeriod;
    /// #
    /// let ts = Timestamp::with_warning_period_opt(
    ///     TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)),
    ///     None
    /// );
    /// assert_eq!(ts.get_kind(), &TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    /// assert_eq!(ts.get_warning_period(), &None);
    ///
    /// let ts = Timestamp::with_warning_period_opt(
    ///     TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)),
    ///     Some(1.day())
    /// );
    /// assert_eq!(ts.get_kind(), &TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    /// assert_eq!(ts.get_warning_period(), &Some(1.day()));
    /// ```
    pub fn with_warning_period_opt(
        kind: TimestampKind,
        warning_period: Option<TimePeriod>,
    ) -> Self {
        Timestamp {
            kind,
            warning_period,
        }
    }

    /// Returns an immutable reference to the kind of this timestamp.
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    /// # use orgmode::TimestampKind;
    /// # use orgmode::AsTimePeriod;
    /// #
    /// let ts = Timestamp::new(TimestampKind::InactiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    /// assert_eq!(ts.get_kind(), &TimestampKind::InactiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    ///
    /// let ts = Timestamp::with_warning_period(
    ///     TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)),
    ///     1.day()
    /// );
    /// assert_eq!(ts.get_kind(), &TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    /// ```
    pub fn get_kind(&self) -> &TimestampKind {
        &self.kind
    }

    /// Returns an immutable reference to the warning period option of this timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    /// # use orgmode::TimestampKind;
    /// # use orgmode::AsTimePeriod;
    /// #
    /// let ts = Timestamp::new(TimestampKind::InactiveDate(NaiveDate::from_ymd(2018, 06, 22)));
    /// assert_eq!(ts.get_warning_period(), &None);
    ///
    /// let ts = Timestamp::with_warning_period(
    ///     TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)),
    ///     1.day()
    /// );
    /// assert_eq!(ts.get_warning_period(), &Some(1.day()));
    /// ```
    pub fn get_warning_period(&self) -> &Option<TimePeriod> {
        &self.warning_period
    }

    /// Returns `true` if the org timestamp is active.
    ///
    /// This is the case if its `kind` is not [`InactiveDate`] or [`InactiveDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    /// # use orgmode::TimestampKind;
    /// #
    /// let x = Timestamp::new(TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 04, 28)));
    /// assert_eq!(x.is_active(), true);
    ///
    /// let x = Timestamp::new(TimestampKind::InactiveDate(NaiveDate::from_ymd(2018, 04, 28)));
    /// assert_eq!(x.is_active(), false);
    /// ```
    ///
    /// [`InactiveDate`]: #variant.InactiveDate
    /// [`InactiveDateTime`]: #variant.InactiveDateTime
    pub fn is_active(&self) -> bool {
        self.kind.is_active()
    }
}

impl FromStr for Timestamp {
    type Err = TimestampParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::types::CompleteStr;
        parse::timestamp(CompleteStr(s))
            .or_else(|err| {
                match err.into_error_kind() {
                    // TODO convert to useful error
                    nom::ErrorKind::Custom(e) => Err(TimestampParseError::Custom(e)),
                    _ => unimplemented!(),
                }
            }).and_then(|(s, ts)| {
                if s == CompleteStr("") {
                    Ok(ts)
                } else {
                    Err(TimestampParseError::TooMuchInput(s.to_string()))
                }
            })
    }
}

#[derive(Debug)]
pub enum TimestampParseError {
    TooMuchInput(String),
    Custom(failure::Error),
}

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

    mod warning_period {
        use super::*;

        #[test]
        fn test_warning_year() {
            assert_eq!(44.year(), TimePeriod::new(44, TimeUnit::Year));
        }

        #[test]
        fn test_warning_month() {
            assert_eq!(44.month(), TimePeriod::new(44, TimeUnit::Month));
        }

        #[test]
        fn test_warning_day() {
            assert_eq!(44.day(), TimePeriod::new(44, TimeUnit::Day));
        }

        #[test]
        fn test_warning_hour() {
            assert_eq!(44.hour(), TimePeriod::new(44, TimeUnit::Hour));
        }
    }

    mod timestamp {
        use super::*;

        #[test]
        fn test_from_str() {
            assert_eq!(
                "<2018-06-13 21:22>".parse().ok(),
                Some(Timestamp::new(TimestampKind::ActiveDatetime(
                    NaiveDate::from_ymd(2018, 06, 13).and_hms(21, 22, 0)
                )))
            );
        }
    }

}
