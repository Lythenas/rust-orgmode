use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use failure::Error;
use std::str;
use std::convert::TryFrom;
use std::fmt;

use Timestamp;
use Repeater;


// Helpers for date and time etc.

/// Checks if the char is a digit in the decimal system (`0` to `9`).
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

/// Converts the given `hour` and `minute` into a `NaiveTime` if possible or gives an error
/// otherwise.
fn naive_time((hour, minute): (&str, &str)) -> Result<NaiveTime, Error> {
    let hour = hour.parse();
    let minute = minute.parse();

    match (hour, minute) {
        (Ok(h), Ok(m)) => {
            NaiveTime::from_hms_opt(h, m, 0).ok_or_else(|| format_err!("invalid time"))
        }
        _ => Err(format_err!("invalid time")),
    }
}

/// Parses a time string in the following format: `12:30` and returns a `NativeTime`.
named!(time<&str, NaiveTime, Error>,
    to_failure!(
        map_res!(
            do_parse!(
                h: take_while_m_n!(2, 2, is_digit) >>
                tag!(":") >>
                m: take_while_m_n!(2, 2, is_digit) >>
                ((h, m))
            ),
            naive_time
        )
    )
);

/// Converts the given `year`, `month`, `day` and optional `weekday` into a `NaiveDate` if possible
/// or gives an error otherwise.
fn naive_date(
    (year, month, day, weekday): (&str, &str, &str, Option<&str>),
) -> Result<NaiveDate, Error> {
    use chrono::Weekday;

    let year = year.parse();
    let month = month.parse();
    let day = day.parse();
    let weekday: Option<Weekday> = match weekday {
        Some(wd) => Some(
            wd.parse()
                .map_err(|_| format_err!("invalid weekday in date"))?,
        ),
        _ => None,
    };

    match (year, month, day) {
        (Ok(y), Ok(m), Ok(d)) => {
            NaiveDate::from_ymd_opt(y, m, d).ok_or_else(|| format_err!("invalid date"))
        }
        _ => Err(format_err!("invalid date")),
    }.and_then(|date| match weekday {
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
                y: take_while_m_n!(4, 4, is_digit) >>
                tag!("-") >>
                m: take_while_m_n!(2, 2, is_digit) >>
                tag!("-") >>
                d: take_while_m_n!(2, 2, is_digit) >>
                wd: opt!(complete!(
                    do_parse!(
                        tag!(" ") >>
                        wd: alt!(tag!("Mon") | tag!("Tue") | tag!("Wed") | tag!("Thu") | 
                                tag!("Fri") | tag!("Sat") | tag!("Sun")) >>
                        (wd)
                    )
                )) >>
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
    InactiveDateWithTimeRange
}

impl fmt::Display for TimestampParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TimestampParseError::*;
        match self {
            InactiveDateWithTimeRange => write!(f, "Found inactive date with a time range. Not allowed.")
        }
    }
}

/// Helper struct for easier parsing.
#[derive(Debug, PartialEq)]
struct Ts {
    active: bool,
    variant: TsVariant,
    repeater: Option<Repeater>,
}

impl <'a> TryFrom<(&'a str, TsVariant, &'a str, Option<TsVariant>)> for Ts {
    type Error = ();

    fn try_from((prefix, variant, suffix, other): (&str, TsVariant, &str, Option<TsVariant>)) -> Result<Self, Self::Error> {
        match (prefix, suffix) {
            ("<", ">") => Ok(true),
            ("[", "]") => Ok(false),
            _ => return Err(()),
        }.and_then(|active| {
            if !active {
                match variant {
                    TsVariant::DateWithTimeRange(_, _, _) => return Err(()),
                    TsVariant::DatetimeRange(_, _) => return Err(()),
                    _ => ()
                };
            }
            Ok(Ts { active, variant, repeater: None })
        }).and_then(|first| Ok(match (first, other) {
            (Ts { active: true, variant: TsVariant::Datetime(start), repeater }, Some(TsVariant::Datetime(end))) => Ts { active: true, variant: TsVariant::DatetimeRange(start, end), repeater },
            (first, None) => first,
            _ => return Err(())
        }))
    }
}

impl TryFrom<Ts> for Timestamp {
    type Error = Error;

    fn try_from(ts: Ts) -> Result<Self, Self::Error> {
        let Ts { active, variant, repeater } = ts;

        if active {
            Ok(match variant {
                TsVariant::Date(date) => Timestamp::ActiveDate(date),
                TsVariant::Datetime(datetime) => Timestamp::ActiveDatetime(datetime),
                TsVariant::DateWithTimeRange(date, start_time, end_time) => Timestamp::TimeRange {
                    date,
                    start_time,
                    end_time,
                },
                TsVariant::DatetimeRange(start_datetime, end_datetime) => Timestamp::DatetimeRange(start_datetime, end_datetime),
            })
        } else {
            Ok(match variant {
                TsVariant::Date(date) => Timestamp::InactiveDate(date),
                TsVariant::Datetime(datetime) => Timestamp::InactiveDatetime(datetime),
                TsVariant::DateWithTimeRange(_, _, _) => return Err(TimestampParseError::InactiveDateWithTimeRange.into()),
                TsVariant::DatetimeRange(start_datetime, end_datetime) => Timestamp::DatetimeRange(start_datetime, end_datetime),
            })
        }
    }
}

//Timestamp::InactiveDate(NaiveDate),
//Timestamp::InactiveDateTime(NaiveDateTime),
//Timestamp::ActiveDate(NaiveDate),
//Timestamp::ActiveDateTime(NaiveDateTime),
//Timestamp::TimeRange {
//    date: NaiveDate,
//    start_time: NaiveTime,
//    end_time: NaiveTime,
//},
//Timestamp::DateRange(NaiveDate, NaiveDate),
//Timestamp::DatetimeRange(NaiveDateTime, NaiveDateTime),
//Timestamp::RepeatingDate(NaiveDate, Duration),
//Timestamp::RepeatingDatetime(NaiveDateTime, Duration),

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
            suffix: to_failure!(alt!(tag!(">") | tag!("]"))) >>
            // TODO repeat
            other: to_failure!(opt!(complete!(do_parse!(
                to_failure!(tag!("--<")) >>
                tsv: tsvariant >>
                to_failure!(tag!(">")) >>
                (tsv)
            )))) >>
            ((prefix, tsv, suffix, other))
        ),
        Ts::try_from
    )
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
                time("12:33").ok(),
                Some(("", NaiveTime::from_hms(12, 33, 0)))
            );
            assert!(time("adadasd").is_err());
            assert!(time("33:99").is_err());
            assert!(time(".1199").is_err());
        }

        #[test]
        fn test_date() {
            assert_eq!(
                date("2018-05-13").ok(),
                Some(("", NaiveDate::from_ymd(2018, 05, 13)))
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
                datetime("2018-05-13 12:40").ok(),
                Some(("", NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)))
            );
            assert_eq!(
                datetime("2018-05-13 Sun 12:40").ok(),
                Some(("", NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)))
            );
            assert!(datetime("aasdadas").is_err());
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
