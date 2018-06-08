use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use failure::Error;
use regex::Regex;
use std::str;

use Timestamp;

lazy_static! {
    static ref REGEX_TIMESTAMP_RANGE: Regex = Regex::new(r"<(.+)>--<(.+)>").unwrap();
}

// Helpers for date and time etc.

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn naive_time((hour, minute): (&str, &str)) -> Result<NaiveTime, &'static str> {
    let hour = hour.parse();
    let minute = minute.parse();
    match (hour, minute) {
        (Ok(h), Ok(m)) => NaiveTime::from_hms_opt(h, m, 0).ok_or("invalid time"),
        _ => Err("invalid time"),
    }
}

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

fn naive_date(
    (year, month, day, weekday): (&str, &str, &str, Option<&str>),
) -> Result<NaiveDate, &'static str> {
    use chrono::Weekday;

    let year = year.parse();
    let month = month.parse();
    let day = day.parse();
    let weekday: Option<Weekday> = match weekday {
        Some(wd) => Some(wd.parse().map_err(|_| "invalid weekday in date")?),
        _ => None,
    };

    match (year, month, day) {
        (Ok(y), Ok(m), Ok(d)) => NaiveDate::from_ymd_opt(y, m, d).ok_or("invalid date"),
        _ => Err("invalid date"),
    }.and_then(|date| match weekday {
        None => Ok(date),
        Some(wd) if wd == date.weekday() => Ok(date),
        _ => Err("invalid weekday in date"),
    })
}

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
                        wd: alt!(tag!("Mon") | tag!("Tue") | tag!("Wed") | tag!("Thu") | tag!("Fri") | tag!("Sat") | tag!("Sun")) >>
                        (wd)
                    )
                )) >>
                ((y, m, d, wd))
            ),
            naive_date
        )
    )
);

named!(datetime<&str, NaiveDateTime, Error>,
    do_parse!(
        date: date >>
        to_failure!(tag!(" ")) >>
        time: time >>
        (date.and_time(time))
    )
);

// combinators to parse actual timestamps

named!(active_date<&str, Timestamp, Error>,
    do_parse!(
        to_failure!(tag!("<")) >>
        date: date >>
        to_failure!(tag!(">")) >>
        (Timestamp::ActiveDate(date))
    )
);

named!(inactive_date<&str, Timestamp, Error>,
    do_parse!(
        to_failure!(tag!("[")) >>
        date: date >>
        to_failure!(tag!("]")) >>
        (Timestamp::InactiveDate(date))
    )
);

named!(active_datetime<&str, Timestamp, Error>,
    do_parse!(
        to_failure!(tag!("<")) >>
        datetime: datetime >>
        to_failure!(tag!(">")) >>
        (Timestamp::ActiveDateTime(datetime))
    )
);

named!(inactive_datetime<&str, Timestamp, Error>,
    do_parse!(
        to_failure!(tag!("[")) >>
        datetime: datetime >>
        to_failure!(tag!("]")) >>
        (Timestamp::InactiveDateTime(datetime))
    )
);

named!(active_time_range<&str, Timestamp, Error>,
    do_parse!(
        to_failure!(tag!("<")) >>
        date: date >>
        to_failure!(tag!(" ")) >>
        start_time: time >>
        to_failure!(tag!("-")) >>
        end_time: time >>
        to_failure!(tag!(">")) >>
        (Timestamp::TimeRange {
            date, start_time, end_time
        })
    )
);

named!(active_datetime_range<&str, Timestamp, Error>,
    do_parse!(
        to_failure!(tag!("<")) >>
        start: datetime >>
        to_failure!(tag!(">--<")) >>
        end: datetime >>
        to_failure!(tag!(">")) >>
        (Timestamp::DateTimeRange(start, end))
    )
);

named!(timestamp<&str, Timestamp, Error>,
       alt!(complete!(call!(active_datetime_range)) | call!(active_date) | call!(active_datetime) | call!(active_time_range) | call!(inactive_date) | call!(inactive_datetime)));

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
                    Timestamp::DateTimeRange(
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
                    Timestamp::DateTimeRange(
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
                    Timestamp::ActiveDateTime(NaiveDate::from_ymd(2018, 06, 13).and_hms(20, 11, 0))
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
                    Timestamp::ActiveDateTime(NaiveDate::from_ymd(2018, 06, 14).and_hms(11, 45, 0))
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
                    Timestamp::InactiveDateTime(
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
                    Timestamp::InactiveDateTime(
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

    //    #[test]
    //    fn test_parse_range_timestamp() {
    //        assert_eq!(
    //            "<2018-06-22 Fri>--<2018-06-23 Sun>".parse(),
    //            Ok(OrgTimestamp::DateRange(
    //                NaiveDate::from_ymd(2018, 6, 22),
    //                NaiveDate::from_ymd(2018, 6, 23)
    //            ))
    //        );
    //        assert_eq!(
    //            "<2018-06-22 Fri 20:00>--<2018-06-23 Sun>".parse(),
    //            Ok(OrgTimestamp::DateTimeRange(
    //                NaiveDate::from_ymd(2018, 6, 22).and_hms(20, 0, 0),
    //                NaiveDate::from_ymd(2018, 6, 23).and_hms(23, 59, 59)
    //            ))
    //        );
    //        assert_eq!(
    //            "<2018-06-22 Fri>--<2018-06-23 Sun 12:30>".parse(),
    //            Ok(OrgTimestamp::DateTimeRange(
    //                NaiveDate::from_ymd(2018, 6, 22).and_hms(0, 0, 0),
    //                NaiveDate::from_ymd(2018, 6, 23).and_hms(12, 30, 0)
    //            ))
    //        );
    //        assert_eq!(
    //            "<2018-06-22 Fri 13:00>--<2018-06-23 Sun 13:00>".parse(),
    //            Ok(OrgTimestamp::DateTimeRange(
    //                NaiveDate::from_ymd(2018, 6, 22).and_hms(13, 0, 0),
    //                NaiveDate::from_ymd(2018, 6, 23).and_hms(13, 0, 0)
    //            ))
    //        );
    //    }
}
