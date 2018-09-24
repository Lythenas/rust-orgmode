//! A timestamp.

use super::*;
use chrono::{NaiveDate, NaiveTime};

/// A timestamp.
///
/// # Semantics
///
/// Timestamps are used in [`elements::Clock`] and [`elements::Planning`] and can occur in normal text.
///
/// They represent a date and time and can be either active or inactive. Usually inactive means
/// that the event is already over or represents the date the event has been dealt with.
///
/// # Syntax
///
/// Follows one of the patterns:
///
/// - diary sexp: `<%%(SEXP)>`
/// - active: `<INNER>`
/// - inactive: `[INNER]`
/// - active range: `<INNER>--<INNER>` or `<DATE TIME-TIME REPEATERORDELAY>`
/// - inactive range: `[INNER]--[INNER]` or `[DATE TIME-TIME REPEATERORDELAY]`
///
/// `SEXP` can contain any character except `>` and newline.
///
/// `INNER` is the pattern `DATE TIME REPEATERORDERLAY`.
///
/// `DATE` follows the pattern `YYYY-MM-DD DAYNAME`. Where `Y`, `M` and `D` are digits
/// (`0`-`9`). `DAYNAME` is optional and can contain any non-whitespace character except `+`,
/// `-`, `]`, `>`, digits and newlines. Usually it is the three letter name of the weekday.
///
/// `TIME` follows the pattern `HH:MM`. Where `H` and `M` are digits. The first `H` can be
/// omitted.
///
/// `REPEATERORDELAY` follows the pattern `MARK VALUE UNIT` where `MARK` is one of `+`, `++`,
/// `.+`, `-` or `--` for the repeat or delay strategy. `VALUE` is a (positive) number. `UNIT`
/// is one of `h`, `d`, `w`, `m` or `y`.
///
/// There can be two `REPEATERORYEAR` in the timestamp. One as a repeater and on as a warning
/// delay.
#[derive(Object, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timestamp {
    shared_behavior_data: SharedBehaviorData,
    pub kind: TimestampKind,
}

impl Timestamp {
    pub fn timestamp_start(&self) -> Option<(&Date, Option<&Time>)> {
        use self::TimestampKind::*;
        use self::TimestampRange::*;

        match &self.kind {
            DiarySexp(_) => None,
            Single(_, TimestampData { date, time, .. })
            | Range(_, DateRange(TimestampData { date, time, .. }, ..)) => {
                Some((&date, time.as_ref()))
            }
            Range(_, TimeRange(TimestampDataWithTime { date, time, .. }, ..)) => {
                Some((&date, Some(&time)))
            }
        }
    }
    pub fn timestamp_end(&self) -> Option<(&Date, Option<&Time>)> {
        use self::TimestampKind::*;
        use self::TimestampRange::*;

        match &self.kind {
            DiarySexp(_) => None,
            Single(_, TimestampData { date, time, .. }) => Some((&date, time.as_ref())),
            Range(_, TimeRange(TimestampDataWithTime { date, .. }, time)) => {
                Some((&date, Some(&time)))
            }
            Range(_, DateRange(_, TimestampData { date, time, .. })) => {
                Some((&date, time.as_ref()))
            }
        }
    }
    pub fn repeater(&self) -> Option<&Repeater> {
        use self::TimestampKind::*;
        use self::TimestampRange::*;

        match &self.kind {
            DiarySexp(_) => None,
            Single(_, TimestampData { repeater, .. })
            | Range(_, TimeRange(TimestampDataWithTime { repeater, .. }, _))
            | Range(_, DateRange(TimestampData { repeater, .. }, _)) => repeater.as_ref(),
        }
    }
    pub fn warning(&self) -> Option<&Warning> {
        use self::TimestampKind::*;
        use self::TimestampRange::*;

        match &self.kind {
            DiarySexp(_) => None,
            Single(_, TimestampData { warning, .. })
            | Range(_, TimeRange(TimestampDataWithTime { warning, .. }, _))
            | Range(_, DateRange(TimestampData { warning, .. }, _)) => warning.as_ref(),
        }
    }
}

/// The kind and date for a [`Timestamp`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimestampKind {
    DiarySexp(String),
    Single(TimestampStatus, TimestampData),
    Range(TimestampStatus, TimestampRange),
}

/// The status of a [`Timestamp`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimestampStatus {
    /// Timestamp in angle brackets (`<...>`).
    Active,
    /// Timestamp in square brackets (`[...]`).
    Inactive,
}

/// The data for a [`TimestampKind`] with optional [`Time`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimestampData {
    pub date: Date,
    pub time: Option<Time>,
    pub repeater: Option<Repeater>,
    pub warning: Option<Warning>,
}

/// A date.
///
/// This is a wrapper around [`chrono::NaiveDate`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date(NaiveDate);

/// A time.
///
/// This is a wrapper around [`chrono::NaiveTime`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Time(NaiveTime);

/// The repeater of a timestamp.
///
/// See [`TimestampData`] and [`TimestampDataWithTime`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repeater {
    pub period: TimePeriod,
    pub strategy: RepeatStrategy,
}

/// The warning delay of a timestamp.
///
/// See [`TimestampData`] and [`TimestampDataWithTime`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Warning {
    pub delay: TimePeriod,
    pub strategy: WarningStrategy,
}

/// The time period (with unit) of a [`Repeater`] or [`Warning`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimePeriod {
    pub value: u32,
    pub unit: TimeUnit,
}

/// The strategy of a [`Repeater`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RepeatStrategy {
    /// Add the repeat duration to the task date once.
    Cumulative,
    /// Add the repeat duration to the task date until the date is in the
    /// future (but at leas once).
    CatchUp,
    /// Add the repeat duration to the current time.
    Restart,
}

/// The strategy of a [`Warning`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WarningStrategy {
    /// Warns for all (repeated) date. Represented as `-` in the org file.
    All,
    /// Warns only for the first date. Represented as `--` in the org file.
    First,
}

/// The unit of a [`TimePeriod`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
}

/// The data for a timestamp range.
///
/// See [`TimestampKind`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimestampRange {
    /// `<DATE TIME-TIME REPEATER-OR-DELAY>` or
    /// `[DATE TIME-TIME REPEATER-OR-DELAY]`
    TimeRange(TimestampDataWithTime, Time),
    /// `<DATE TIME REPEATER-OR-DELAY>--<DATE TIME REPEATER-OR-DELAY>` or
    /// `[DATE TIME REPEATER-OR-DELAY]--[DATE TIME REPEATER-OR-DELAY]`
    DateRange(TimestampData, TimestampData),
}

/// The data for a timestamp with a time.
///
/// See [`TimestampRange`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimestampDataWithTime {
    pub date: Date,
    pub time: Time,
    pub repeater: Option<Repeater>,
    pub warning: Option<Warning>,
}
