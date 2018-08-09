use chrono::{Duration, NaiveDate, NaiveTime};
use failure::Error;
use std::convert::TryFrom;
use std::fmt;
use std::str::{self, FromStr};

use timestamp::{*, Date};

use nom::types::CompleteStr;

// Helpers for date and time etc.

/// Checks if the char is a digit in the decimal system (`0` to `9`).
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(parse_u32<CompleteStr, u32, Error>,
    to_failure!(map_res!(
        take_while1!(is_digit),
        |s: CompleteStr| u32::from_str(*s)
    ))
);

named!(parse_i32<CompleteStr, i32, Error>,
    to_failure!(map_res!(
        recognize!(do_parse!(
            opt!(alt!(tag!("-") | tag!("+"))) >>
            take_while1!(is_digit) >>
            ()
        )),
        |s: CompleteStr| i32::from_str_radix(*s, 10)
            .map_err(|_| format_err!("invalid i32"))
    ))
);

/// Converts the given `hour` and `minute` into `Time` if possible
/// or gives an error otherwise.
fn to_time((hour, minute): (u32, u32)) -> Result<Time, Error> {
    NaiveTime::from_hms_opt(hour, minute, 0).ok_or_else(|| format_err!("invalid time")).map(Time::new)
}

/// Parses a time string in the following format: `12:30` and returns
/// a `NaiveTime`.
named!(time<CompleteStr, Time, Error>,
    to_failure!(
        map_res!(
            do_parse!(
                h: parse_u32 >>
                to_failure!(tag!(":")) >>
                m: parse_u32 >>
                ((h, m))
            ),
            to_time
        )
    )
);

/// Converts the given `year`, `month`, `day` and optional `weekday` into
/// a `Date` if possible or gives an error otherwise.
fn to_date(
    (year, month, day, weekday): (i32, u32, u32, Option<&str>),
) -> Result<Date, Error> {
    use chrono::{Weekday, Datelike};

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
/// E.g. `2018-06-30` or `2018-06-30 Sat`.
named!(date<CompleteStr, Date, Error>,
    to_failure!(
        map_res!(
            do_parse!(
                year: parse_i32 >>
                to_failure!(tag!("-")) >>
                month: parse_u32 >>
                to_failure!(tag!("-")) >>
                day: parse_u32 >>
                dayname: to_failure!(opt!(complete!(
                    do_parse!(
                        tag!(" ") >>
                        dayname: alt!(
                            tag!("Mon") | tag!("Tue") | tag!("Wed")
                            | tag!("Thu") | tag!("Fri")
                            | tag!("Sat") | tag!("Sun")
                        ) >>
                        (dayname)
                    )
                ))) >>
                ((year, month, day, dayname.map(|s| *s)))
            ),
            to_date
        )
    )
);

#[derive(Debug, PartialEq, Fail)]
enum TimestampParseError {
    InvalidRepeater,
    InvalidWarning,
    InvalidCompoundTimestamp,
}

// needed to derive Fail
impl fmt::Display for TimestampParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO write actual error messages
        write!(f, "{:?}", self)
    }
}

fn check_active(prefix: &str, suffix: &str) -> Result<bool, ()> {
    match (prefix, suffix) {
        ("<", ">") => Ok(true),
        ("[", "]") => Ok(false),
        _ => Err(()),
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
named!(repeater<CompleteStr, Repeater, Error>,
    to_failure!(do_parse!(
        strategy: repeat_strategy >>
        time_period: time_period >>
        (Repeater::new(time_period, strategy))
    ))
);

/// Parses a [`RepeatStrategy`].
named!(repeat_strategy<CompleteStr, RepeatStrategy, Error>,
    to_failure!(
        map_res!(
            alt!(
                tag!("++") |
                tag!("+") |
                tag!(".+")
            ),
            cstr(self::to_repeat_strategy)
        )
    )
);

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
fn cstr<T>(f: impl Fn(&str) -> T) -> impl Fn(CompleteStr) -> T {
    move |s| f(*s)
}

/// Parses a `TimeUnit` using its `from_str`-method if there is a
/// valid character.
named!(time_unit<CompleteStr, TimeUnit, Error>,
    to_failure!(map_res!(
        alt!(tag!("y") | tag!("m") | tag!("w") | tag!("d") | tag!("h")),
        cstr(TimeUnit::from_str)
    ))
);

/// Parses a [`TimePeriod`].
named!(time_period<CompleteStr, TimePeriod, Error>,
    to_failure!(do_parse!(
        value: to_failure!(parse_u32) >>
        unit: time_unit >>
        (TimePeriod::new(value, unit))
    ))
);

/// Parses a [`WarningStrategy`].
named!(warning_strategy<CompleteStr, WarningStrategy, Error>,
    to_failure!(
        map_res!(
            alt!(
                tag!("++") |
                tag!("+") |
                tag!(".+")
            ),
            cstr(self::to_warning_strategy)
        )
    )
);

/// Converts the given str to a [`WarningStrategy`] if possible.
fn to_warning_strategy(s: &str) -> Result<WarningStrategy, Error> {
    match s {
        "-" => Ok(WarningStrategy::All),
        "--" => Ok(WarningStrategy::First),
        _ => Err(TimestampParseError::InvalidWarning.into()),
    }
}

/// Parses a [`WarningDelay`].
named!(warning_delay<CompleteStr, WarningDelay, Error>,
    to_failure!(do_parse!(
        warning_strategy: warning_strategy >>
        time_period: time_period >>
        (WarningDelay::new(time_period, warning_strategy))
    ))
);

/// Parses a `(Option<Repeater>, Option<WarningDelay>)`.
named!(repeater_and_delay<CompleteStr,
       (Option<Repeater>, Option<WarningDelay>), Error>,
    to_failure!(do_parse!(
        // repeater and warning delay can be flipped
        repeater1: opt!(repeater) >>
        warning_delay: opt!(warning_delay) >>
        repeater2: opt!(repeater) >>
        ((repeater1.or(repeater2), warning_delay))
    ))
);

/// Parses a [`TimestampData`]. E.g. `DATE TIME[-TIME] REPEATER-OR-DELAY`
/// with optional second time for a time range.
named!(inner_timestamp<CompleteStr, (TimestampData, Option<Time>), Error>,
    to_failure!(do_parse!(
        date: date >>
        time1: to_failure!(opt!(time)) >>
        time2: to_failure!(opt!(do_parse!(
            to_failure!(tag!("-")) >>
            time: time >>
            (time)
        ))) >>
        repeater_and_delay: repeater_and_delay >>
        (to_timestamp_data(date, time1, repeater_and_delay), time2)
    ))
);

fn to_timestamp_data(date: Date, time: Option<Time>, (repeater, delay): (Option<Repeater>, Option<WarningDelay>)) -> TimestampData {
    TimestampData::new(date).and_opt_time(time).and_opt_repeater(repeater).and_opt_warning_delay(delay)
}

/// Parses a single timestamp.
///
/// Which is one of
///
/// * `<DATE TIME REPEATER-OR-DELAY>`
/// * `[DATE TIME REPEATER-OR-DELAY]`
/// * `<DATE TIME-TIME REPEATER-OR-DELAY>`
/// * `[DATE TIME-TIME REPEATER-OR-DELAY]`
named!(single_timestamp<CompleteStr, Timestamp, Error>,
    to_failure!(do_parse!(
        prefix: to_failure!(alt!(tag!("<") | tag!("["))) >>
        inner_timestamp: inner_timestamp >>
        suffix: to_failure!(switch!(value!(prefix),
            CompleteStr("<") => tag!(">") |
            CompleteStr("[") => tag!("]")
        )) >>
        (self::to_single_timestamp(*prefix, inner_timestamp))
    ))
);

fn to_single_timestamp(
    prefix: &str,
    (timestamp_data, end_time): (TimestampData, Option<Time>),
) -> Timestamp {
    if prefix == "<" {
        // active
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
                TimestampDataWithTime::with_everything(
                    timestamp_data.get_date().clone(),
                    start_time.clone(),
                    timestamp_data.get_repeater().clone(),
                    timestamp_data.get_warning_delay().clone()
                ),
                end_time
            ))
        } else {
            None
        }
    } else {
        None
    }
}

named!(pub timestamp<CompleteStr, Timestamp, Error>,
    to_failure!(map_res!(
        to_failure!(do_parse!(
            first: single_timestamp >>
            second: to_failure!(opt!(do_parse!(
                to_failure!(tag!("--")) >>
                timestamp: single_timestamp >>
                (timestamp)
            ))) >>
            ((first, second))
        )),
        self::to_timestamp
    ))
);

fn to_timestamp((start, end): (Timestamp, Option<Timestamp>)) -> Result<Timestamp, Error> {
    use Timestamp::*;
    match (start, end) {
        (Active(start), Some(Active(end))) => Ok(ActiveRange(TimestampRange::DateRange(start, end))),
        (Inactive(start), Some(Inactive(end))) => Ok(InactiveRange(TimestampRange::DateRange(start, end))),
        (start, None) => Ok(start),
        (_, _) => Err(TimestampParseError::InvalidCompoundTimestamp.into())
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
    /// assert_ts!("str to parse" => Timestamp::new(TimestampKind::Date...));
    /// ```
    ///
    /// Testing somthing should parse with rest:
    ///
    /// ```ignore
    /// assert_ts!("str to parse with rest" => "with rest", Timestamp::new(TimestampKind::Date...));
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
                Some((CompleteStr(">"), NaiveTime::from_hms(12, 33, 0)))
            );
            assert!(time(CompleteStr("adadasd")).is_err());
            assert!(time(CompleteStr("33:99")).is_err());
            assert!(time(CompleteStr(".1199")).is_err());
        }

        #[test]
        fn test_date() {
            assert_eq!(
                date(CompleteStr("2018-05-13>")).ok(),
                Some((CompleteStr(">"), NaiveDate::from_ymd(2018, 05, 13)))
            );
            assert_eq!(
                date(CompleteStr("2018-05-13 Sun")).ok(),
                Some((CompleteStr(""), NaiveDate::from_ymd(2018, 05, 13)))
            );
            assert!(date(CompleteStr("adadasd")).is_err());
        }

        #[test]
        fn test_datetime() {
            assert_eq!(
                datetime(CompleteStr("2018-05-13 12:40>")).ok(),
                Some((
                    CompleteStr(">"),
                    NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)
                ))
            );
            assert_eq!(
                datetime(CompleteStr("2018-05-13 Sun 12:40>")).ok(),
                Some((
                    CompleteStr(">"),
                    NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)
                ))
            );
            assert!(datetime(CompleteStr("aasdadas")).is_err());
        }
    }

    mod repeater_with_warning {
        use super::*;
        use RepeatStrategy::*;

        #[test]
        fn test_add_once_with_warning() {
            assert_ts!(
                "<2018-06-04 12:55 +1d -1h>" =>
                Timestamp::with_warning_period(TimestampKind::RepeatingDatetime(
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                    Repeater::new(1.day(), Cumulative)
                ), 1.hour())
            );
            assert_ts!(
                "<2018-06-04 12:55 +1d -1h>" =>
                Timestamp::with_warning_period(TimestampKind::RepeatingDatetime(
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                    Repeater::new(1.day(), Cumulative)
                ), 1.hour())
            );
            assert_ts!(
                "<2018-06-04 +1w -1d>" =>
                Timestamp::with_warning_period(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(1.week(), Cumulative)
                ), 1.day())
            );
        }
    }

    mod repeater {
        use super::*;
        use RepeatStrategy::*;

        #[test]
        fn test_add_once() {
            assert_ts!(
                "<2018-06-04 12:55 +1w>" =>
                Timestamp::new(TimestampKind::RepeatingDatetime(
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                    Repeater::new(1.week(), Cumulative)
                ))
            );
            assert_ts!(
                "<2018-06-04 +1w>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(1.week(), Cumulative)
                ))
            );
            assert_ts!(
                "<2018-06-04 +20d>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(20.day(), Cumulative)
                ))
            );
            assert_ts!(
                "<2018-06-04 +5h>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(5.hour(), Cumulative)
                ))
            );
            assert_ts!(
                "<2018-06-04 +7m>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(7.month(), Cumulative)
                ))
            );
            assert_ts!(
                "<2018-06-04 +1y>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(1.year(), Cumulative)
                ))
            );
        }

        #[test]
        fn test_add_until_future() {
            assert_ts!(
                "<2018-06-04 12:55 ++1w>" =>
                Timestamp::new(TimestampKind::RepeatingDatetime(
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                    Repeater::new(1.week(), CatchUp)
                ))
            );
            assert_ts!(
                "<2018-06-04 ++1w>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(1.week(), CatchUp)
                ))
            );
            assert_ts!(
                "<2018-06-04 ++20d>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(20.day(), CatchUp)
                ))
            );
            assert_ts!(
                "<2018-06-04 ++5h>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(5.hour(), CatchUp)
                ))
            );
            assert_ts!(
                "<2018-06-04 ++20m>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(20.month(), CatchUp)
                ))
            );
            assert_ts!(
                "<2018-06-04 ++5y>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(5.year(), CatchUp)
                ))
            );
        }

        #[test]
        fn test_add_to_now() {
            assert_ts!(
                "<2018-06-04 12:55 .+1w>" =>
                Timestamp::new(TimestampKind::RepeatingDatetime(
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                    Repeater::new(1.week(), Restart)
                ))
            );
            assert_ts!(
                "<2018-06-04 .+1w>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(1.week(), Restart)
                ))
            );
            assert_ts!(
                "<2018-06-04 .+20d>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(20.day(), Restart)
                ))
            );
            assert_ts!(
                "<2018-06-04 .+5h>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(5.hour(), Restart)
                ))
            );
            assert_ts!(
                "<2018-06-04 .+2m>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(2.month(), Restart)
                ))
            );
            assert_ts!(
                "<2018-06-04 .+12y>" =>
                Timestamp::new(TimestampKind::RepeatingDate(
                    NaiveDate::from_ymd(2018, 06, 04),
                    Repeater::new(12.year(), Restart)
                ))
            );
        }
    }

    mod active_datetimerange {
        use super::*;

        #[test]
        fn test_same_day() {
            assert_ts!(
                "<2018-06-04 12:00>--<2018-06-04 14:00>" =>
                Timestamp::new(TimestampKind::DatetimeRange(
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 0, 0),
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(14, 0, 0)
                ))
            );
            assert_ts!(
                "<2018-07-18 Wed 12:00>--<2018-07-18 Wed 14:00>" =>
                Timestamp::new(TimestampKind::DatetimeRange(
                    NaiveDate::from_ymd(2018, 07, 18).and_hms(12, 0, 0),
                    NaiveDate::from_ymd(2018, 07, 18).and_hms(14, 0, 0)
                ))
            );
        }

        #[test]
        fn test_different_days() {
            assert_ts!(
                "<2018-06-04 12:00>--<2018-08-09 11:54>" =>
                Timestamp::new(TimestampKind::DatetimeRange(
                    NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 0, 0),
                    NaiveDate::from_ymd(2018, 08, 09).and_hms(11, 54, 0)
                ))
            );
            assert_ts!(
                "<2018-07-18 Wed 12:00>--<2018-07-20 Fri 11:54>" =>
                Timestamp::new(TimestampKind::DatetimeRange(
                    NaiveDate::from_ymd(2018, 07, 18).and_hms(12, 0, 0),
                    NaiveDate::from_ymd(2018, 07, 20).and_hms(11, 54, 0)
                ))
            );
        }
    }

    mod active_timerange {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_ts!(
                "<2018-06-04 Mon 13:00-14:30>" =>
                Timestamp::new(TimestampKind::TimeRange {
                    date: NaiveDate::from_ymd(2018, 06, 04),
                    start_time: NaiveTime::from_hms(13, 0, 0),
                    end_time: NaiveTime::from_hms(14, 30, 0)
                })
            );
        }

        #[test]
        fn without_weekday() {
            assert_ts!(
                "<2018-06-04 13:00-14:30>" =>
                Timestamp::new(TimestampKind::TimeRange {
                    date: NaiveDate::from_ymd(2018, 06, 04),
                    start_time: NaiveTime::from_hms(13, 0, 0),
                    end_time: NaiveTime::from_hms(14, 30, 0)
                })
            );
        }
    }

    mod active_datetime {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_ts!(
                "<2018-06-13 Wed 20:11>" =>
                Timestamp::new(TimestampKind::ActiveDatetime(NaiveDate::from_ymd(2018, 06, 13).and_hms(20, 11, 0)))
            );
            assert_ts!("<2018-06-13 Mon 11:33>" => #);
        }

        #[test]
        fn without_weekday() {
            assert_ts!(
                "<2018-06-14 11:45>" =>
                Timestamp::new(TimestampKind::ActiveDatetime(NaiveDate::from_ymd(2018, 06, 14).and_hms(11, 45, 0)))
            );
        }

    }

    mod active_date {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_ts!(
                "<2018-06-13 Wed>" =>
                Timestamp::new(TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 13)))
            );
            assert_ts!("<2018-06-13 Mon>" => #);
        }

        #[test]
        fn without_weekday() {
            assert_ts!(
                "<2018-06-22>" =>
                Timestamp::new(TimestampKind::ActiveDate(NaiveDate::from_ymd(2018, 06, 22)))
            );
        }

    }

    mod active_daterange {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_ts!(
                "<2018-06-13 Wed>--<2018-06-16 Sat>" =>
                Timestamp::new(TimestampKind::DateRange(NaiveDate::from_ymd(2018, 06, 13), NaiveDate::from_ymd(2018, 06, 16)))
            );
        }

        #[test]
        fn without_weekday() {
            assert_ts!(
                "<2018-06-13>--<2018-06-16>" =>
                Timestamp::new(TimestampKind::DateRange(NaiveDate::from_ymd(2018, 06, 13), NaiveDate::from_ymd(2018, 06, 16)))
            );
            assert_ts!(
                "<2018-06-13 Wed>--<2018-06-16>" =>
                Timestamp::new(TimestampKind::DateRange(NaiveDate::from_ymd(2018, 06, 13), NaiveDate::from_ymd(2018, 06, 16)))
            );
            assert_ts!(
                "<2018-06-13>--<2018-06-16 Sat>" =>
                Timestamp::new(TimestampKind::DateRange(NaiveDate::from_ymd(2018, 06, 13), NaiveDate::from_ymd(2018, 06, 16)))
            );
        }
    }

    mod inactive_datetime {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_ts!(
                "[2018-06-13 Wed 11:13]" =>
                Timestamp::new(TimestampKind::InactiveDatetime(
                    NaiveDate::from_ymd(2018, 06, 13).and_hms(11, 13, 0)
                ))
            );
            assert_ts!("[2018-06-13 Mon 11:13]" => #);
        }

        #[test]
        fn without_weekday() {
            assert_ts!(
                "[2018-06-13 11:39]" =>
                Timestamp::new(TimestampKind::InactiveDatetime(
                    NaiveDate::from_ymd(2018, 06, 13).and_hms(11, 39, 0)
                ))
            );
        }

    }

    mod inactive_date {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_ts!(
                "[2018-06-13 Wed]" =>
                Timestamp::new(TimestampKind::InactiveDate(NaiveDate::from_ymd(2018, 06, 13)))
            );
            assert_ts!("[2018-06-13 Mon]" => #);
        }

        #[test]
        fn without_weekday() {
            assert_ts!(
                "[2018-06-13]" =>
                Timestamp::new(TimestampKind::InactiveDate(NaiveDate::from_ymd(2018, 06, 13)))
            );
        }

    }

}
