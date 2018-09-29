use super::{OrgInput, OrgResult};
use chrono::{NaiveDate, NaiveTime};
use crate::{
    Date, RepeatStrategy, Repeater, Time, TimePeriod, TimeUnit, Timestamp, TimestampData,
    TimestampDataWithTime, TimestampRange, WarningDelay, WarningStrategy,
};
use failure::Error;
use nom::types::CompleteStr;
use std::fmt;
use std::str::{self, FromStr};

// TODO add better error returns to the parsers.
// e.g. with return_error! or add_return_error!.

// Helpers for date and time etc.

/// Checks if the char is a digit in the decimal system (`0` to `9`).
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

/// Parses an u32.
fn parse_u32(i: OrgInput<'_>) -> OrgResult<'_, u32> {
    to_failure!(
        i,
        map_res!(take_while1!(is_digit), |s: CompleteStr<'_>| u32::from_str(
            *s
        ))
    )
}

/// Parses an i32.
fn parse_i32(i: OrgInput<'_>) -> OrgResult<'_, i32> {
    to_failure!(
        i,
        map_res!(
            recognize!(do_parse!(
                opt!(alt!(tag!("-") | tag!("+"))) >> take_while1!(is_digit) >> ()
            )),
            |s: CompleteStr<'_>| i32::from_str_radix(*s, 10)
                .map_err(|_| format_err!("invalid i32"))
        )
    )
}

/// Converts the given `hour` and `minute` into `Time` if possible
/// or gives an error otherwise.
fn to_time((hour, minute): (u32, u32)) -> Result<Time, Error> {
    NaiveTime::from_hms_opt(hour, minute, 0)
        .ok_or_else(|| format_err!("invalid time"))
        .map(Time::new)
}

/// Parses a time string in the following format: `12:30` and returns
/// a `NaiveTime`.
fn time(i: OrgInput<'_>) -> OrgResult<'_, Time> {
    to_failure!(
        i,
        map_res!(
            do_parse!(h: parse_u32 >> to_failure!(tag!(":")) >> m: parse_u32 >> ((h, m))),
            to_time
        )
    )
}

/// Converts the given `year`, `month`, `day` and optional `weekday` into
/// a `Date` if possible or gives an error otherwise.
fn to_date((year, month, day, weekday): (i32, u32, u32, Option<&str>)) -> Result<Date, Error> {
    use chrono::{Datelike, Weekday};

    let weekday: Option<Weekday> = match weekday {
        Some(wd) => Some(
            wd.parse()
                .map_err(|_| format_err!("invalid weekday in date"))?,
        ),
        _ => None,
    };

    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| format_err!("invalid date"))
        .and_then(|date| match weekday {
            None => Ok(date),
            Some(wd) if wd == date.weekday() => Ok(date),
            _ => Err(format_err!("invalid weekday in date")),
        })
        .map(Date::new)
}

/// Parses a date string in the format `YYYY-MM-DD DAYNAME` and returns
/// a `NaiveDate`. The dayname is optional.
///
/// E.g. `2018-06-30` or `2018-06-30 Sat`."],
fn date(i: OrgInput<'_>) -> OrgResult<'_, Date> {
    to_failure!(
        i,
        map_res!(
            do_parse!(
                year: parse_i32
                    >> to_failure!(tag!("-"))
                    >> month: parse_u32
                    >> to_failure!(tag!("-"))
                    >> day: parse_u32
                    >> dayname:
                        to_failure!(opt!(complete!(do_parse!(
                            tag!(" ")
                                >> dayname:
                                    alt!(
                                        tag!("Mon")
                                            | tag!("Tue")
                                            | tag!("Wed")
                                            | tag!("Thu")
                                            | tag!("Fri")
                                            | tag!("Sat")
                                            | tag!("Sun")
                                    )
                                >> (dayname)
                        ))))
                    >> ((year, month, day, dayname.map(|s| *s)))
            ),
            to_date
        )
    )
}

#[derive(Debug, PartialEq, Fail)]
enum TimestampParseError {
    InvalidRepeater,
    InvalidWarning,
    InvalidCompoundTimestamp,
}

// needed to derive Fail
impl fmt::Display for TimestampParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO write actual error messages
        write!(f, "{:?}", self)
    }
}

impl FromStr for TimeUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "y" => TimeUnit::Year,
            "m" => TimeUnit::Month,
            "w" => TimeUnit::Week,
            "d" => TimeUnit::Day,
            "h" => TimeUnit::Hour,
            _ => return Err(TimestampParseError::InvalidRepeater.into()),
        })
    }
}

impl From<(RepeatStrategy, u32, TimeUnit)> for Repeater {
    fn from((strategy, value, unit): (RepeatStrategy, u32, TimeUnit)) -> Self {
        Repeater::new(TimePeriod::new(value, unit), strategy)
    }
}

/// Parses a [`Repeater`].
fn repeater(i: OrgInput<'_>) -> OrgResult<'_, Repeater> {
    to_failure!(
        i,
        do_parse!(
            strategy: repeat_strategy
                >> time_period: time_period
                >> (Repeater::new(time_period, strategy))
        )
    )
}

/// Parses a [`RepeatStrategy`].
fn repeat_strategy(i: OrgInput<'_>) -> OrgResult<'_, RepeatStrategy> {
    to_failure!(
        i,
        map_res!(
            alt!(tag!("++") | tag!("+") | tag!(".+")),
            cstr(self::to_repeat_strategy)
        )
    )
}

/// Converts the given str to a [`RepeatStrategy`] if possible.
fn to_repeat_strategy(s: &str) -> Result<RepeatStrategy, Error> {
    match s {
        "+" => Ok(RepeatStrategy::Cumulative),
        "++" => Ok(RepeatStrategy::CatchUp),
        ".+" => Ok(RepeatStrategy::Restart),
        _ => Err(TimestampParseError::InvalidRepeater.into()),
    }
}

impl From<(u32, TimeUnit)> for TimePeriod {
    fn from((value, unit): (u32, TimeUnit)) -> Self {
        TimePeriod::new(value, unit)
    }
}

/// Helper function to convert a `Fn(&str) -> T` to `Fn(CompleteStr) -> T`.
fn cstr<T>(f: impl Fn(&str) -> T) -> impl Fn(CompleteStr<'_>) -> T {
    move |s| f(*s)
}

/// Parses a `TimeUnit` using its `from_str`-method if there is a
/// valid character.
fn time_unit(i: OrgInput<'_>) -> OrgResult<'_, TimeUnit> {
    to_failure!(
        i,
        map_res!(
            alt!(tag!("y") | tag!("m") | tag!("w") | tag!("d") | tag!("h")),
            cstr(TimeUnit::from_str)
        )
    )
}

/// Parses a [`TimePeriod`].
fn time_period(i: OrgInput<'_>) -> OrgResult<'_, TimePeriod> {
    to_failure!(
        i,
        do_parse!(
            value: to_failure!(parse_u32) >> unit: time_unit >> (TimePeriod::new(value, unit))
        )
    )
}

/// Parses a [`WarningStrategy`].
fn warning_strategy(i: OrgInput<'_>) -> OrgResult<'_, WarningStrategy> {
    to_failure!(
        i,
        map_res!(
            alt!(tag!("-") | tag!("--")),
            cstr(self::to_warning_strategy)
        )
    )
}

/// Converts the given str to a [`WarningStrategy`] if possible.
fn to_warning_strategy(s: &str) -> Result<WarningStrategy, Error> {
    match s {
        "-" => Ok(WarningStrategy::All),
        "--" => Ok(WarningStrategy::First),
        _ => Err(TimestampParseError::InvalidWarning.into()),
    }
}

/// Parses a [`WarningDelay`].
fn warning_delay(i: OrgInput<'_>) -> OrgResult<'_, WarningDelay> {
    to_failure!(
        i,
        do_parse!(
            warning_strategy: warning_strategy
                >> time_period: time_period
                >> (WarningDelay::new(time_period, warning_strategy))
        )
    )
}

/// Parses a `(Option<Repeater>, Option<WarningDelay>)`.
fn repeater_and_delay(i: OrgInput<'_>) -> OrgResult<'_, (Option<Repeater>, Option<WarningDelay>)> {
    to_failure!(
        i,
        do_parse!(
            // repeater and warning delay can be flipped
            repeater1: opt!(preceded!(to_failure!(tag!(" ")), repeater))
                >> warning_delay: opt!(preceded!(to_failure!(tag!(" ")), warning_delay))
                >> repeater2: opt!(preceded!(to_failure!(tag!(" ")), repeater))
                >> ((repeater1.or(repeater2), warning_delay))
        )
    )
}

/// Parses a [`TimestampData`]. E.g. `DATE TIME[-TIME] REPEATER-OR-DELAY`
/// with optional second time for a time range.
fn inner_timestamp(i: OrgInput<'_>) -> OrgResult<'_, (TimestampData, Option<Time>)> {
    to_failure!(
        i,
        do_parse!(
            date: date
                >> time1:
                    to_failure!(opt!(do_parse!(
                        to_failure!(tag!(" ")) >> time: time >> (time)
                    )))
                >> time2:
                    to_failure!(opt!(do_parse!(
                        to_failure!(tag!("-")) >> time: time >> (time)
                    )))
                >> repeater_and_delay: repeater_and_delay
                >> (to_timestamp_data(date, time1, repeater_and_delay), time2)
        )
    )
}

/// Converts a date and optional time, repeater and warning delay to [`TimestampData`].
fn to_timestamp_data(
    date: Date,
    time: Option<Time>,
    (repeater, delay): (Option<Repeater>, Option<WarningDelay>),
) -> TimestampData {
    TimestampData::new(date)
        .and_opt_time(time)
        .and_opt_repeater(repeater)
        .and_opt_warning_delay(delay)
}

/// Parses a single timestamp.
///
/// Which is one of
///
/// - `<DATE TIME REPEATER-OR-DELAY>`
/// - `[DATE TIME REPEATER-OR-DELAY]`
/// - `<DATE TIME-TIME REPEATER-OR-DELAY>`
/// - `[DATE TIME-TIME REPEATER-OR-DELAY]`
fn single_timestamp(i: OrgInput<'_>) -> OrgResult<'_, Timestamp> {
    to_failure!(
        i,
        do_parse!(
            prefix: to_failure!(alt!(tag!("<") | tag!("[")))
                >> inner_timestamp: inner_timestamp
                >> to_failure!(switch!(value!(prefix),
            CompleteStr("<") => tag!(">") |
            CompleteStr("[") => tag!("]")
        )) >> (self::to_single_timestamp(*prefix == "<", inner_timestamp))
        )
    )
}

/// Converts timestamp data and optional time to a single (active or inactive) [`Timestamp`].
/// This can also be a time range.
fn to_single_timestamp(
    active: bool,
    (timestamp_data, end_time): (TimestampData, Option<Time>),
) -> Timestamp {
    if active {
        match to_timestamp_range_time_range(&timestamp_data, end_time) {
            Some(range) => Timestamp::ActiveRange(range),
            None => Timestamp::Active(timestamp_data),
        }
    } else {
        // inactive
        match to_timestamp_range_time_range(&timestamp_data, end_time) {
            Some(range) => Timestamp::InactiveRange(range),
            None => Timestamp::Inactive(timestamp_data),
        }
    }
}

/// Converts timestamp data and a second optional time into a
/// [`TimestampRange::TimeRange`] if possible.
fn to_timestamp_range_time_range(
    timestamp_data: &TimestampData,
    end_time: Option<Time>,
) -> Option<TimestampRange> {
    if let Some(end_time) = end_time {
        if let Some(start_time) = timestamp_data.get_time() {
            // TODO maybe check if end time is greater than start time
            Some(TimestampRange::TimeRange(
                TimestampDataWithTime::new(timestamp_data.get_date().clone(), start_time.clone())
                    .and_opt_repeater(timestamp_data.get_repeater().clone())
                    .and_opt_warning_delay(timestamp_data.get_warning_delay().clone()),
                end_time,
            ))
        } else {
            None
        }
    } else {
        None
    }
}

/// Parses a complete timestamp in one of the accepted format.

/// See [`Timestamp`].
pub fn timestamp(i: OrgInput<'_>) -> OrgResult<'_, Timestamp> {
    to_failure!(
        i,
        map_res!(
            to_failure!(do_parse!(
                first: single_timestamp
                    >> second:
                        to_failure!(opt!(do_parse!(
                            to_failure!(tag!("--")) >> timestamp: single_timestamp >> (timestamp)
                        )))
                    >> ((first, second))
            )),
            self::to_timestamp
        )
    )
}

/// Converts two single timestamps (the second is optional) to complete [`Timestamp`] if possible.
///
/// It can't be converted e.g. when one of the timestamps is already a time range but both are
/// given. (`<2018-06-20 12:30-14:00>--<2018-07-01 22:00>`)
fn to_timestamp((start, end): (Timestamp, Option<Timestamp>)) -> Result<Timestamp, Error> {
    use crate::Timestamp::*;
    match (start, end) {
        (Active(start), Some(Active(end))) => {
            Ok(ActiveRange(TimestampRange::DateRange(start, end)))
        }
        (Inactive(start), Some(Inactive(end))) => {
            Ok(InactiveRange(TimestampRange::DateRange(start, end)))
        }
        (start, None) => Ok(start),
        (_, _) => Err(TimestampParseError::InvalidCompoundTimestamp.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper macro for testing.
    ///
    /// # Usage
    ///
    /// Testing something should fail to parse:
    ///
    /// ```
    /// assert_ts!("garbage" => #);
    /// ```
    ///
    /// Testing something should completely parse:
    ///
    /// ```ignore
    /// assert_ts!("str to parse" => Timestamp::Active(
    ///     TimestampData::new(...)...
    /// ));
    /// ```
    ///
    /// Testing somthing should parse with rest:
    ///
    /// ```ignore
    /// assert_ts!("str to parse with rest" =>
    ///     "with rest",
    ///     Timestamp::Active(TimestampData::new(...)...)
    /// );
    /// ```
    macro_rules! assert_ts {
        ($str:expr => #) => {{
            assert!(timestamp(CompleteStr($str)).is_err())
        }};
        ($str:expr => $res:expr) => {{
            assert_ts!($str => "", $res)
        }};
        ($str:expr => $rem:expr, $res:expr) => {{
            // Can't compare the entire Result with Ok(...)
            // because the Error type does not implement PartialEq
            assert_eq!(
                timestamp(CompleteStr($str)).ok(),
                Some((CompleteStr($rem), $res)),
                "Parsing of {:?} failed.",
                $str
            )
        }};
    }

    macro_rules! assert_ts_many {
        ($str:expr => $res:expr) => {{
            assert_ts!($str => $res);
        }};
        ($str:expr => $res:expr; $($rest:tt)* ) => {{
            assert_ts!($str => $res);
            assert_ts_many!($($rest)*);
        }};
    }

    mod helpers {
        use super::*;

        #[test]
        fn test_parse_u32() {
            assert_eq!(
                parse_u32(CompleteStr("55")).ok(),
                Some((CompleteStr(""), 55))
            );
            assert_eq!(
                parse_u32(CompleteStr("199a")).ok(),
                Some((CompleteStr("a"), 199))
            );
            assert!(parse_u32(CompleteStr("err")).is_err());
        }

        #[test]
        fn test_parse_i32() {
            assert_eq!(
                parse_i32(CompleteStr("55")).ok(),
                Some((CompleteStr(""), 55))
            );
            assert_eq!(
                parse_i32(CompleteStr("199a")).ok(),
                Some((CompleteStr("a"), 199))
            );
            assert_eq!(
                parse_i32(CompleteStr("-2501")).ok(),
                Some((CompleteStr(""), -2501))
            );
            assert_eq!(
                parse_i32(CompleteStr("+2015x")).ok(),
                Some((CompleteStr("x"), 2015))
            );
            assert!(parse_i32(CompleteStr("err")).is_err());
            assert!(parse_i32(CompleteStr("+err")).is_err());
            assert!(parse_i32(CompleteStr("-err")).is_err());
        }

        #[test]
        fn test_time() {
            assert_eq!(
                time(CompleteStr("12:33>")).ok(),
                Some((CompleteStr(">"), NaiveTime::from_hms(12, 33, 0).into()))
            );
            assert!(time(CompleteStr("adadasd")).is_err());
            assert!(time(CompleteStr("33:99")).is_err());
            assert!(time(CompleteStr(".1199")).is_err());
        }

        #[test]
        fn test_date() {
            assert_eq!(
                date(CompleteStr("2018-05-13>")).ok(),
                Some((CompleteStr(">"), NaiveDate::from_ymd(2018, 05, 13).into()))
            );
            assert_eq!(
                date(CompleteStr("2018-05-13 Sun")).ok(),
                Some((CompleteStr(""), NaiveDate::from_ymd(2018, 05, 13).into()))
            );
            assert!(date(CompleteStr("adadasd")).is_err());
        }
    }

    // TODO test smaller parts (repeater, warning_delay, ...)

    mod timestamp {
        use super::*;
        use crate::AsTimePeriod;

        #[test]
        fn date_only() {
            let expected = Timestamp::Active(TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)));
            assert_ts_many!(
                "<2018-08-04>" =>
                expected.clone();
                "<2018-08-04 Sat>" =>
                expected.clone()
            );
            let expected =
                Timestamp::Inactive(TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)));
            assert_ts_many!(
                "[2018-08-04]" =>
                expected.clone();
                "[2018-08-04 Sat]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_repeater() {
            let expected = Timestamp::Active(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative)),
            );
            assert_ts_many!(
                "<2018-08-04 +1h>" =>
                expected.clone();
                "<2018-08-04 Sat +1h>" =>
                expected.clone()
            );
            let expected = Timestamp::Inactive(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative)),
            );
            assert_ts_many!(
                "[2018-08-04 +1h]" =>
                expected.clone();
                "[2018-08-04 Sat +1h]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_warning() {
            let expected = Timestamp::Active(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "<2018-08-04 -1h>" =>
                expected.clone();
                "<2018-08-04 Sat -1h>" =>
                expected.clone()
            );
            let expected = Timestamp::Inactive(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "[2018-08-04 -1h]" =>
                expected.clone();
                "[2018-08-04 Sat -1h]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_repeater_and_warning() {
            let expected = Timestamp::Active(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "<2018-08-04 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 Sat +1h -1h>" =>
                expected.clone();
                "<2018-08-04 -1h +1h>" =>
                expected.clone();
                "<2018-08-04 Sat -1h +1h>" =>
                expected.clone()
            );
            let expected = Timestamp::Inactive(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "[2018-08-04 +1h -1h]" =>
                expected.clone();
                "[2018-08-04 Sat +1h -1h]" =>
                expected.clone();
                "[2018-08-04 -1h +1h]" =>
                expected.clone();
                "[2018-08-04 Sat -1h +1h]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time() {
            let expected = Timestamp::Active(TimestampData::with_time(
                NaiveDate::from_ymd(2018, 08, 04),
                NaiveTime::from_hms(12, 0, 0),
            ));
            assert_ts_many!(
                "<2018-08-04 12:00>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00>" =>
                expected.clone()
            );
            let expected = Timestamp::Inactive(TimestampData::with_time(
                NaiveDate::from_ymd(2018, 08, 04),
                NaiveTime::from_hms(12, 0, 0),
            ));
            assert_ts_many!(
                "[2018-08-04 12:00]" =>
                expected.clone();
                "[2018-08-04 Sat 12:00]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time_and_repeater() {
            let expected = Timestamp::Active(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative)),
            );
            assert_ts_many!(
                "<2018-08-04 12:00 +1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00 +1h>" =>
                expected.clone()
            );
            let expected = Timestamp::Inactive(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative)),
            );
            assert_ts_many!(
                "[2018-08-04 12:00 +1h]" =>
                expected.clone();
                "[2018-08-04 Sat 12:00 +1h]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time_and_warning() {
            let expected = Timestamp::Active(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "<2018-08-04 12:00 -1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00 -1h>" =>
                expected.clone()
            );
            let expected = Timestamp::Inactive(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "[2018-08-04 12:00 -1h]" =>
                expected.clone();
                "[2018-08-04 Sat 12:00 -1h]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time_and_repeater_and_warning() {
            let expected = Timestamp::Active(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "<2018-08-04 12:00 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 12:00 -1h +1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00 -1h +1h>" =>
                expected.clone()
            );
            let expected = Timestamp::Inactive(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            );
            assert_ts_many!(
                "[2018-08-04 12:00 +1h -1h]" =>
                expected.clone();
                "[2018-08-04 12:00 -1h +1h]" =>
                expected.clone();
                "[2018-08-04 Sat 12:00 +1h -1h]" =>
                expected.clone();
                "[2018-08-04 Sat 12:00 -1h +1h]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time_range() {
            let expected = Timestamp::ActiveRange(TimestampRange::TimeRange(
                TimestampDataWithTime::new(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                ),
                NaiveTime::from_hms(13, 0, 0).into(),
            ));
            assert_ts_many!(
                "<2018-08-04 12:00-13:00>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00-13:00>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time_range_and_repeater() {
            let expected = Timestamp::ActiveRange(TimestampRange::TimeRange(
                TimestampDataWithTime::new(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative)),
                NaiveTime::from_hms(13, 0, 0).into(),
            ));
            assert_ts_many!(
                "<2018-08-04 12:00-13:00 +1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00-13:00 +1h>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time_range_and_warning() {
            let expected = Timestamp::ActiveRange(TimestampRange::TimeRange(
                TimestampDataWithTime::new(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
                NaiveTime::from_hms(13, 0, 0).into(),
            ));
            assert_ts_many!(
                "<2018-08-04 12:00-13:00 -1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00-13:00 -1h>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_time_range_and_repeater_and_warning() {
            let expected = Timestamp::ActiveRange(TimestampRange::TimeRange(
                TimestampDataWithTime::new(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
                NaiveTime::from_hms(13, 0, 0).into(),
            ));
            assert_ts_many!(
                "<2018-08-04 12:00-13:00 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00-13:00 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 12:00-13:00 -1h +1h>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00-13:00 -1h +1h>" =>
                expected.clone()
            );
            let expected = Timestamp::InactiveRange(TimestampRange::TimeRange(
                TimestampDataWithTime::new(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                )
                .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
                NaiveTime::from_hms(13, 0, 0).into(),
            ));
            assert_ts_many!(
                "[2018-08-04 12:00-13:00 +1h -1h]" =>
                expected.clone();
                "[2018-08-04 Sat 12:00-13:00 +1h -1h]" =>
                expected.clone();
                "[2018-08-04 12:00-13:00 -1h +1h]" =>
                expected.clone();
                "[2018-08-04 Sat 12:00-13:00 -1h +1h]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06)),
            ));
            assert_ts_many!(
                "<2018-08-04>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04>--<2018-08-06 Mon>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 Mon>" =>
                expected.clone()
            );
            let expected = Timestamp::InactiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06)),
            ));
            assert_ts_many!(
                "[2018-08-04]--[2018-08-06]" =>
                expected.clone();
                "[2018-08-04 Sat]--[2018-08-06]" =>
                expected.clone();
                "[2018-08-04]--[2018-08-06 Mon]" =>
                expected.clone();
                "[2018-08-04 Sat]--[2018-08-06 Mon]" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_start_time() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                ),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06)),
            ));
            assert_ts_many!(
                "<2018-08-04 12:00>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 12:00>--<2018-08-06 Mon>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00>--<2018-08-06 Mon>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_end_time() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)),
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 06),
                    NaiveTime::from_hms(13, 0, 0),
                ),
            ));
            assert_ts_many!(
                "<2018-08-04>--<2018-08-06 13:00>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 13:00>" =>
                expected.clone();
                "<2018-08-04>--<2018-08-06 Mon 13:00>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 Mon 13:00>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_start_and_end_time() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 04),
                    NaiveTime::from_hms(12, 0, 0),
                ),
                TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 08, 06),
                    NaiveTime::from_hms(13, 0, 0),
                ),
            ));
            assert_ts_many!(
                "<2018-08-04 12:00>--<2018-08-06 13:00>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00>--<2018-08-06 13:00>" =>
                expected.clone();
                "<2018-08-04 12:00>--<2018-08-06 Mon 13:00>" =>
                expected.clone();
                "<2018-08-04 Sat 12:00>--<2018-08-06 Mon 13:00>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_start_repeater() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06)),
            ));
            assert_ts_many!(
                "<2018-08-04 +1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 Sat +1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 +1h>--<2018-08-06 Mon>" =>
                expected.clone();
                "<2018-08-04 Sat +1h>--<2018-08-06 Mon>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_start_warning() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06)),
            ));
            assert_ts_many!(
                "<2018-08-04 -1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 Sat -1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 -1h>--<2018-08-06 Mon>" =>
                expected.clone();
                "<2018-08-04 Sat -1h>--<2018-08-06 Mon>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_start_repeater_and_warning() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06)),
            ));
            assert_ts_many!(
                "<2018-08-04 +1h -1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 Sat +1h -1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 +1h -1h>--<2018-08-06 Mon>" =>
                expected.clone();
                "<2018-08-04 Sat +1h -1h>--<2018-08-06 Mon>" =>
                expected.clone();
                "<2018-08-04 -1h +1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 Sat -1h +1h>--<2018-08-06>" =>
                expected.clone();
                "<2018-08-04 -1h +1h>--<2018-08-06 Mon>" =>
                expected.clone();
                "<2018-08-04 Sat -1h +1h>--<2018-08-06 Mon>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_end_repeater() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative)),
            ));
            assert_ts_many!(
                "<2018-08-04>--<2018-08-06 +1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 +1h>" =>
                expected.clone();
                "<2018-08-04>--<2018-08-06 Mon +1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 Mon +1h>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_end_warning() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            ));
            assert_ts_many!(
                "<2018-08-04>--<2018-08-06 -1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 -1h>" =>
                expected.clone();
                "<2018-08-04>--<2018-08-06 Mon -1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 Mon -1h>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_end_repeater_and_warning() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            ));
            assert_ts_many!(
                "<2018-08-04>--<2018-08-06 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 +1h -1h>" =>
                expected.clone();
                "<2018-08-04>--<2018-08-06 Mon +1h -1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 Mon +1h -1h>" =>
                expected.clone();
                "<2018-08-04>--<2018-08-06 -1h +1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 -1h +1h>" =>
                expected.clone();
                "<2018-08-04>--<2018-08-06 Mon -1h +1h>" =>
                expected.clone();
                "<2018-08-04 Sat>--<2018-08-06 Mon -1h +1h>" =>
                expected.clone()
            );
        }

        #[test]
        fn with_date_range_and_start_repeater_and_warning_and_end_repeater_and_warning() {
            let expected = Timestamp::ActiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            ));
            assert_ts_many!(
                "<2018-08-04 +1h -1h>--<2018-08-06 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 Sat +1h -1h>--<2018-08-06 +1h -1h>" =>
                expected.clone();
                "<2018-08-04 +1h -1h>--<2018-08-06 Mon +1h -1h>" =>
                expected.clone();
                "<2018-08-04 Sat +1h -1h>--<2018-08-06 Mon +1h -1h>" =>
                expected.clone()
            );
            let expected = Timestamp::InactiveRange(TimestampRange::DateRange(
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 04))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
                TimestampData::new(NaiveDate::from_ymd(2018, 08, 06))
                    .and_repeater(Repeater::new(1.hour(), RepeatStrategy::Cumulative))
                    .and_warning_delay(WarningDelay::new(1.hour(), WarningStrategy::All)),
            ));
            assert_ts_many!(
                "[2018-08-04 +1h -1h]--[2018-08-06 +1h -1h]" =>
                expected.clone();
                "[2018-08-04 Sat +1h -1h]--[2018-08-06 +1h -1h]" =>
                expected.clone();
                "[2018-08-04 +1h -1h]--[2018-08-06 Mon +1h -1h]" =>
                expected.clone();
                "[2018-08-04 Sat +1h -1h]--[2018-08-06 Mon +1h -1h]" =>
                expected.clone()
            );
        }

        // Potentially more tests but I'm confident that this works
        //fn with_date_range_and_start_repeater_and_end_repeater() {}
        //fn with_date_range_and_start_repeater_and_end_warning() {}
        //fn with_date_range_and_start_repeater_and_end_repeater_and_warning() {}
        //fn with_date_range_and_start_warning_and_end_repeater() {}
        //fn with_date_range_and_start_warning_and_end_warning() {}
        //fn with_date_range_and_start_warning_and_end_repeater_and_warning() {}
        //fn with_date_range_and_start_repeater_and_warning_and_end_repeater() {}
        //fn with_date_range_and_start_repeater_and_warning_and_end_warning() {}
        //...
    }
}
