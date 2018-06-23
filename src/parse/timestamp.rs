use chrono::prelude::*;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use failure::Error;
use std::convert::TryFrom;
use std::fmt;
use std::str::{self, FromStr};

use RepeatStrategy;
use Repeater;
use TimeUnit;
use Timestamp;
use WarningPeriod;

// Helpers for date and time etc.

/// Checks if the char is a digit in the decimal system (`0` to `9`).
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(parse_u32<&str, u32, Error>,
    to_failure!(map_res!(take_while1!(is_digit), u32::from_str))
);

#[test]
fn test_parse_u32() {
    assert_eq!(parse_u32("55>").ok(), Some((">", 55)))
}

fn from_dec(s: &str) -> Result<i32, Error> {
    i32::from_str_radix(s, 10).map_err(|_| format_err!("invalid i32"))
}

named!(parse_i32<&str, i32, Error>,
    to_failure!(map_res!(recognize!(do_parse!(
        opt!(alt!(tag!("-") | tag!("+"))) >>
        take_while1!(is_digit) >>
        ()
    )), from_dec))
);

/// Converts the given `hour` and `minute` into a `NaiveTime` if possible or gives an error
/// otherwise.
fn naive_time((hour, minute): (u32, u32)) -> Result<NaiveTime, Error> {
    NaiveTime::from_hms_opt(hour, minute, 0).ok_or_else(|| format_err!("invalid time"))
}

/// Parses a time string in the following format: `12:30` and returns a `NativeTime`.
named!(time<&str, NaiveTime, Error>,
    to_failure!(
        map_res!(
            do_parse!(
                h: parse_u32 >>
                to_failure!(tag!(":")) >>
                m: parse_u32 >>
                ((h, m))
            ),
            naive_time
        )
    )
);

/// Converts the given `year`, `month`, `day` and optional `weekday` into a `NaiveDate` if possible
/// or gives an error otherwise.
fn naive_date(
    (year, month, day, weekday): (u32, u32, u32, Option<&str>),
) -> Result<NaiveDate, Error> {
    use chrono::Weekday;

    let weekday: Option<Weekday> = match weekday {
        Some(wd) => Some(
            wd.parse()
                .map_err(|_| format_err!("invalid weekday in date"))?,
        ),
        _ => None,
    };

    let year = i32::try_from(year).unwrap();

    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| format_err!("invalid date"))
        .and_then(|date| match weekday {
            None => Ok(date),
            Some(wd) if wd == date.weekday() => Ok(date),
            _ => Err(format_err!("invalid weekday in date")),
        })
}

/// Parses a date string in the following format: `2018-06-30` or `2018-06-30 Sat` and returns a
/// `NaiveDate`.
named!(date<&str, NaiveDate, Error>,
    to_failure!(
        map_res!(
            do_parse!(
                y: parse_u32 >>
                to_failure!(tag!("-")) >>
                m: parse_u32 >>
                to_failure!(tag!("-")) >>
                d: parse_u32 >>
                wd: to_failure!(opt!(complete!(
                    do_parse!(
                        tag!(" ") >>
                        wd: alt!(tag!("Mon") | tag!("Tue") | tag!("Wed")
                                 | tag!("Thu") | tag!("Fri")
                                 | tag!("Sat") | tag!("Sun")) >>
                        (wd)
                    )
                ))) >>
                ((y, m, d, wd))
            ),
            naive_date
        )
    )
);

/// Parses a datetime string in the following format: `2018-06-30 Sat 12:30` (weekday optional) and
/// returns a `NaiveDateTime`.
named!(datetime<&str, NaiveDateTime, Error>,
    do_parse!(
        date: date >>
        to_failure!(tag!(" ")) >>
        time: time >>
        (date.and_time(time))
    )
);

#[derive(Debug, PartialEq, Fail)]
enum TimestampParseError {
    InactiveDateWithTimeRange,
    InactiveDateWithRepeater,
    RangedDateWithRepeater,
    InvalidRepeater,
}

impl fmt::Display for TimestampParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TimestampParseError::*;
        match self {
            InactiveDateWithTimeRange => {
                write!(f, "Found inactive date with a time range. Not allowed.")
            }
            InactiveDateWithRepeater => {
                write!(f, "Found inactive date with a repeater. Not allowed.")
            }
            RangedDateWithRepeater => {
                write!(f, "Found time/datetime range with repeater. Not allowed.")
            }
            InvalidRepeater => write!(f, "Found invalid repeater."),
        }
    }
}

/// Helper struct for easier parsing.
#[derive(Debug, PartialEq)]
struct Ts {
    active: bool,
    variant: TsVariant,
    repeater: Option<Repeater>,
    warning: Option<WarningPeriod>,
}

impl<'a>
    TryFrom<(
        &'a str,
        TsVariant,
        Option<Repeater>,
        Option<WarningPeriod>,
        &'a str,
        Option<TsVariant>,
    )> for Ts
{
    type Error = ();

    fn try_from(
        (prefix, variant, repeater, warning, suffix, other): (
            &str,
            TsVariant,
            Option<Repeater>,
            Option<WarningPeriod>,
            &str,
            Option<TsVariant>,
        ),
    ) -> Result<Self, Self::Error> {
        match (prefix, suffix) {
            ("<", ">") => Ok(true),
            ("[", "]") => Ok(false),
            _ => return Err(()),
        }.and_then(|active| {
            if !active {
                match variant {
                    TsVariant::DateWithTimeRange(_, _, _) => return Err(()),
                    TsVariant::DatetimeRange(_, _) => return Err(()),
                    _ => (),
                };
            }
            Ok(Ts {
                active,
                variant,
                repeater,
                warning,
            })
        })
            .and_then(|first| {
                Ok(match (first, other) {
                    (
                        Ts {
                            active: true,
                            variant: TsVariant::Datetime(start),
                            repeater,
                            warning,
                        },
                        Some(TsVariant::Datetime(end)),
                    ) => Ts {
                        active: true,
                        variant: TsVariant::DatetimeRange(start, end),
                        repeater,
                        warning,
                    },
                    (first, None) => first,
                    _ => return Err(()),
                })
            })
    }
}

impl TryFrom<Ts> for Timestamp {
    type Error = Error;

    fn try_from(ts: Ts) -> Result<Self, Self::Error> {
        let Ts {
            active,
            variant,
            repeater,
            warning,
        } = ts;

        if let Some(repeater) = repeater {
            if !active {
                return Err(TimestampParseError::InactiveDateWithRepeater.into());
            }

            return match variant {
                TsVariant::Date(date) => Ok(Timestamp::RepeatingDate(date, repeater)),
                TsVariant::Datetime(datetime) => {
                    Ok(Timestamp::RepeatingDatetime(datetime, repeater))
                }
                _ => Err(TimestampParseError::RangedDateWithRepeater.into()),
            };
        }

        if active {
            Ok(match variant {
                TsVariant::Date(date) => Timestamp::ActiveDate(date),
                TsVariant::Datetime(datetime) => Timestamp::ActiveDatetime(datetime),
                TsVariant::DateWithTimeRange(date, start_time, end_time) => Timestamp::TimeRange {
                    date,
                    start_time,
                    end_time,
                },
                TsVariant::DatetimeRange(start_datetime, end_datetime) => {
                    Timestamp::DatetimeRange(start_datetime, end_datetime)
                }
            })
        } else {
            Ok(match variant {
                TsVariant::Date(date) => Timestamp::InactiveDate(date),
                TsVariant::Datetime(datetime) => Timestamp::InactiveDatetime(datetime),
                TsVariant::DateWithTimeRange(_, _, _) => {
                    return Err(TimestampParseError::InactiveDateWithTimeRange.into())
                }
                TsVariant::DatetimeRange(start_datetime, end_datetime) => {
                    Timestamp::DatetimeRange(start_datetime, end_datetime)
                }
            })
        }
    }
}

/// Helper enum for easier parsing.
#[derive(Debug, PartialEq)]
enum TsVariant {
    Date(NaiveDate),
    Datetime(NaiveDateTime),
    DateWithTimeRange(NaiveDate, NaiveTime, NaiveTime),
    DatetimeRange(NaiveDateTime, NaiveDateTime),
}

impl TsVariant {
    fn from(date: NaiveDate, time_range: Option<(NaiveTime, Option<NaiveTime>)>) -> Self {
        match time_range {
            None => TsVariant::Date(date),
            Some((time, None)) => TsVariant::Datetime(date.and_time(time)),
            Some((start_time, Some(end_time))) => {
                TsVariant::DateWithTimeRange(date, start_time, end_time)
            }
        }
    }
}

named!(ts<&str, Ts, Error>,
    map_res!(
        do_parse!(
            prefix: to_failure!(alt!(tag!("<") | tag!("["))) >>
            tsv: tsvariant >>
            repeater: opt!(repeater) >>
            warning: opt!(warning_period) >>
            suffix: to_failure!(alt!(tag!(">") | tag!("]"))) >>
            other: to_failure!(opt!(complete!(do_parse!(
                to_failure!(tag!("--<")) >>
                tsv: tsvariant >>
                to_failure!(tag!(">")) >>
                (tsv)
            )))) >>
            ((prefix, tsv, repeater, warning, suffix, other))
        ),
        Ts::try_from
    )
);

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
    fn from((strategy, amount, unit): (RepeatStrategy, u32, TimeUnit)) -> Self {
        Repeater {
            amount,
            unit,
            strategy,
        }
    }
}

named!(repeater<&str, Repeater, Error>,
    to_failure!(map_res!(do_parse!(
        to_failure!(tag!(" ")) >>
        strategy: repeat_strategy >>
        amount: parse_u32 >>
        unit: time_unit >>
        ((strategy, amount, unit))
    ), Repeater::try_from))
);

named!(repeat_strategy<&str, RepeatStrategy, Error>,
    to_failure!(
        map_res!(alt!(tag!("++") | tag!("+") | tag!(".+")), to_strategy)
    )
);

/// Converts the given str to a `RepeatStrategy` if possible.
fn to_strategy(s: &str) -> Result<RepeatStrategy, Error> {
    match s {
        "+" => Ok(RepeatStrategy::AddOnce),
        "++" => Ok(RepeatStrategy::AddUntilFuture),
        ".+" => Ok(RepeatStrategy::AddToNow),
        _ => Err(TimestampParseError::InvalidRepeater.into()),
    }
}

impl From<(u32, TimeUnit)> for WarningPeriod {
    fn from((amount, unit): (u32, TimeUnit)) -> Self {
        WarningPeriod {
            amount: amount,
            unit,
        }
    }
}

named!(time_unit<&str, TimeUnit, Error>,
    to_failure!(map_res!(
        alt!(tag!("y") | tag!("m") | tag!("w") | tag!("d") | tag!("h")),
        TimeUnit::from_str
    ))
);

named!(warning_period<&str, WarningPeriod, Error>,
    to_failure!(do_parse!(
        to_failure!(tag!(" -")) >>
        amount: to_failure!(parse_u32) >>
        unit: time_unit >>
        (WarningPeriod { amount, unit, })
    ))
);

named!(tsvariant<&str, TsVariant, Error>,
    to_failure!(do_parse!(
        date: date >>
        time_range: to_failure!(opt!(do_parse!(
            to_failure!(tag!(" ")) >>
            start_time: time >>
            end_time: to_failure!(opt!(do_parse!(
                to_failure!(tag!("-")) >>
                time: time >>
                (time)
            ))) >>
            ((start_time, end_time))
        ))) >>
        (TsVariant::from(date, time_range))
    ))
);

named!(timestamp<&str, Timestamp, Error>,
    map_res!(
        ts,
        TryFrom::try_from
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    mod helpers {
        use super::*;

        #[test]
        fn test_time() {
            assert_eq!(
                time("12:33>").ok(),
                Some((">", NaiveTime::from_hms(12, 33, 0)))
            );
            assert!(time("adadasd").is_err());
            assert!(time("33:99").is_err());
            assert!(time(".1199").is_err());
        }

        #[test]
        fn test_date() {
            assert_eq!(
                date("2018-05-13>").ok(),
                Some((">", NaiveDate::from_ymd(2018, 05, 13)))
            );
            assert_eq!(
                date("2018-05-13 Sun").ok(),
                Some(("", NaiveDate::from_ymd(2018, 05, 13)))
            );
            assert!(date("adadasd").is_err());
        }

        #[test]
        fn test_datetime() {
            assert_eq!(
                datetime("2018-05-13 12:40>").ok(),
                Some((">", NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)))
            );
            assert_eq!(
                datetime("2018-05-13 Sun 12:40>").ok(),
                Some((">", NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)))
            );
            assert!(datetime("aasdadas").is_err());
        }
    }

    mod repeater_with_warning {
        use super::*;

        // TODO
    }

    mod repeater {
        use super::*;

        #[test]
        fn test_add_once() {
            assert_eq!(
                timestamp("<2018-06-04 12:55 +1w>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDatetime(
                        NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                        Repeater {
                            amount: 1,
                            unit: TimeUnit::Week,
                            strategy: RepeatStrategy::AddOnce
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 +1w>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 1,
                            unit: TimeUnit::Week,
                            strategy: RepeatStrategy::AddOnce
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 +20d>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 20,
                            unit: TimeUnit::Day,
                            strategy: RepeatStrategy::AddOnce
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 +5h>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 5,
                            unit: TimeUnit::Hour,
                            strategy: RepeatStrategy::AddOnce
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 +7m>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 7,
                            unit: TimeUnit::Month,
                            strategy: RepeatStrategy::AddOnce
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 +1y>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 1,
                            unit: TimeUnit::Year,
                            strategy: RepeatStrategy::AddOnce
                        }
                    )
                ))
            );
        }

        #[test]
        fn test_add_until_future() {
            assert_eq!(
                timestamp("<2018-06-04 12:55 ++1w>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDatetime(
                        NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                        Repeater {
                            amount: 1,
                            unit: TimeUnit::Week,
                            strategy: RepeatStrategy::AddUntilFuture
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 ++1w>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 1,
                            unit: TimeUnit::Week,
                            strategy: RepeatStrategy::AddUntilFuture
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 ++20d>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 20,
                            unit: TimeUnit::Day,
                            strategy: RepeatStrategy::AddUntilFuture
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 ++5h>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 5,
                            unit: TimeUnit::Hour,
                            strategy: RepeatStrategy::AddUntilFuture
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 ++20m>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 20,
                            unit: TimeUnit::Month,
                            strategy: RepeatStrategy::AddUntilFuture
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 ++5y>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 5,
                            unit: TimeUnit::Year,
                            strategy: RepeatStrategy::AddUntilFuture
                        }
                    )
                ))
            );
        }

        #[test]
        fn test_add_to_now() {
            assert_eq!(
                timestamp("<2018-06-04 12:55 .+1w>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDatetime(
                        NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 55, 0),
                        Repeater {
                            amount: 1,
                            unit: TimeUnit::Week,
                            strategy: RepeatStrategy::AddToNow
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 .+1w>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 1,
                            unit: TimeUnit::Week,
                            strategy: RepeatStrategy::AddToNow
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 .+20d>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 20,
                            unit: TimeUnit::Day,
                            strategy: RepeatStrategy::AddToNow
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 .+5h>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 5,
                            unit: TimeUnit::Hour,
                            strategy: RepeatStrategy::AddToNow
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 .+2m>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 2,
                            unit: TimeUnit::Month,
                            strategy: RepeatStrategy::AddToNow
                        }
                    )
                ))
            );
            assert_eq!(
                timestamp("<2018-06-04 .+12y>").ok(),
                Some((
                    "",
                    Timestamp::RepeatingDate(
                        NaiveDate::from_ymd(2018, 06, 04),
                        Repeater {
                            amount: 12,
                            unit: TimeUnit::Year,
                            strategy: RepeatStrategy::AddToNow
                        }
                    )
                ))
            );
        }
    }

    mod active_datetimerange {
        use super::*;

        #[test]
        fn test_same_day() {
            assert_eq!(
                timestamp("<2018-06-04 12:00>--<2018-06-04 14:00>").ok(),
                Some((
                    "",
                    Timestamp::DatetimeRange(
                        NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 0, 0),
                        NaiveDate::from_ymd(2018, 06, 04).and_hms(14, 0, 0)
                    )
                ))
            );
        }

        #[test]
        fn test_different_days() {
            assert_eq!(
                timestamp("<2018-06-04 12:00>--<2018-08-09 11:54>").ok(),
                Some((
                    "",
                    Timestamp::DatetimeRange(
                        NaiveDate::from_ymd(2018, 06, 04).and_hms(12, 0, 0),
                        NaiveDate::from_ymd(2018, 08, 09).and_hms(11, 54, 0)
                    )
                ))
            );
        }
    }

    mod active_timerange {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_eq!(
                timestamp("<2018-06-04 Mon 13:00-14:30>").ok(),
                Some((
                    "",
                    Timestamp::TimeRange {
                        date: NaiveDate::from_ymd(2018, 06, 04),
                        start_time: NaiveTime::from_hms(13, 0, 0),
                        end_time: NaiveTime::from_hms(14, 30, 0)
                    }
                ))
            );
        }

    }

    mod active_datetime {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_eq!(
                timestamp("<2018-06-13 Wed 20:11>").ok(),
                Some((
                    "",
                    Timestamp::ActiveDatetime(NaiveDate::from_ymd(2018, 06, 13).and_hms(20, 11, 0))
                ))
            );
            assert!(timestamp("<2018-06-13 Mon 11:33>").is_err());
        }

        #[test]
        fn without_weekday() {
            assert_eq!(
                timestamp("<2018-06-14 11:45>").ok(),
                Some((
                    "",
                    Timestamp::ActiveDatetime(NaiveDate::from_ymd(2018, 06, 14).and_hms(11, 45, 0))
                ))
            );
        }

    }

    mod active_date {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_eq!(
                timestamp("<2018-06-13 Wed>").ok(),
                Some(("", Timestamp::ActiveDate(NaiveDate::from_ymd(2018, 06, 13))))
            );
            assert!(timestamp("<2018-06-13 Mon>").is_err());
        }

        #[test]
        fn without_weekday() {
            assert_eq!(
                timestamp("<2018-06-22>").ok(),
                Some(("", Timestamp::ActiveDate(NaiveDate::from_ymd(2018, 06, 22))))
            );
        }

    }

    mod inactive_datetime {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_eq!(
                timestamp("[2018-06-13 Wed 11:13]").ok(),
                Some((
                    "",
                    Timestamp::InactiveDatetime(
                        NaiveDate::from_ymd(2018, 06, 13).and_hms(11, 13, 0)
                    )
                ))
            );
            assert!(timestamp("[2018-06-13 Mon 11:13]").is_err());
        }

        #[test]
        fn without_weekday() {
            assert_eq!(
                timestamp("[2018-06-13 11:39]").ok(),
                Some((
                    "",
                    Timestamp::InactiveDatetime(
                        NaiveDate::from_ymd(2018, 06, 13).and_hms(11, 39, 0)
                    )
                ))
            );
        }

    }

    mod inactive_date {
        use super::*;

        #[test]
        fn with_weekday() {
            assert_eq!(
                timestamp("[2018-06-13 Wed]").ok(),
                Some((
                    "",
                    Timestamp::InactiveDate(NaiveDate::from_ymd(2018, 06, 13))
                ))
            );
            assert!(timestamp("[2018-06-13 Mon]").is_err());
        }

        #[test]
        fn without_weekday() {
            assert_eq!(
                timestamp("[2018-06-13]").ok(),
                Some((
                    "",
                    Timestamp::InactiveDate(NaiveDate::from_ymd(2018, 06, 13))
                ))
            );
        }

    }

}
