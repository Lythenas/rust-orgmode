use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use failure::Error;
use nom::{is_alphabetic, is_digit, IResult};
use regex::Regex;
use std::str;

use Timestamp;

/// Helper enum for easier matching in the parse functions
enum SimpleTimestamp {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

lazy_static! {
    static ref REGEX_TIMESTAMP_RANGE: Regex = Regex::new(r"<(.+)>--<(.+)>").unwrap();
    static ref REGEX_TIMESTAMP_INACTIVE: Regex = Regex::new(r"\[(.+)\]").unwrap();
    static ref REGEX_TIMESTAMP_ACTIVE: Regex = Regex::new(r"<(.+)>").unwrap();
    static ref REGEX_DATE: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})( (?P<weekday>[A-Z][a-z]{2}))?( (?P<rest>.*))?").unwrap();
    static ref REGEX_TIME: Regex = Regex::new(r"(?P<hours>\d{2}):(?P<minutes>\d{2})").unwrap();
    static ref REGEX_TIME_RANGE: Regex = Regex::new(r"(?P<start_hours>\d{2}):(?P<start_minutes>\d{2})-(?P<end_hours>\d{2}):(?P<end_minutes>\d{2})").unwrap();
}

named!(
    in_angle_brackets<&str, &str>,
    delimited!(tag!("<"), take_until!(">"), tag!(">"))
);

named!(
    in_square_brackets<&str, &str>,
    delimited!(char!('['), is_not!("]"), char!(']'))
);

// TODO do date and time parsing manually. This makes e.g. ranges easier

fn date_with_weekday(s: &str) -> IResult<&str, NaiveDate> {
    match NaiveDate::parse_from_str(s, "%Y-%m-%d %a") {
        Ok(d) => Ok(("", d)),
        Err(e) => Err(::nom::Err::Error(error_position!(
            s,
            ::nom::ErrorKind::Custom(0)
        ))),
    }
}

fn date_without_weekday(s: &str) -> IResult<&str, NaiveDate> {
    match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Ok(d) => Ok(("", d)),
        Err(e) => Err(::nom::Err::Error(error_position!(
            s,
            ::nom::ErrorKind::Custom(0)
        ))),
    }
}

named!(date<&str, NaiveDate>,
    alt!(call!(date_with_weekday) | call!(date_without_weekday))
);

fn datetime_with_weekday(s: &str) -> IResult<&str, NaiveDateTime, Error> {
    match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %a %H:%M") {
        Ok(d) => Ok(("", d)),
        Err(e) => Err(::nom::Err::Error(error_position!(
            s,
            ::nom::ErrorKind::Custom(e.into())
        ))),
    }
}

fn datetime_without_weekday(s: &str) -> IResult<&str, NaiveDateTime, Error> {
    match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
        Ok(d) => Ok(("", d)),
        Err(e) => Err(::nom::Err::Error(error_position!(
            s,
            ::nom::ErrorKind::Custom(e.into())
        ))),
    }
}

named!(datetime<&str, NaiveDateTime, Error>,
       alt!(call!(datetime_with_weekday) | call!(datetime_without_weekday)));

named!(active_datetime<&str, Timestamp, Error>,
    do_parse!(
        s: u32_to_failure!(in_angle_brackets) >>
        ts: u32_to_failure!(expr_res!(datetime(s))) >>
        (Timestamp::ActiveDateTime(ts.1))
    )
);

named!(inactive_datetime<&str, Timestamp, Error>,
    do_parse!(
        s: u32_to_failure!(call!(in_square_brackets)) >>
        ts: u32_to_failure!(expr_res!(datetime(s))) >>
        (Timestamp::InactiveDateTime(ts.1))
    )
);

named!(active_date<&str, Timestamp, Error>,
    do_parse!(
        s: u32_to_failure!(in_angle_brackets) >>
        ts: u32_to_failure!(expr_res!(date(s))) >>
        (Timestamp::ActiveDate(ts.1))
    )
);

named!(inactive_date<&str, Timestamp, Error>,
    do_parse!(
        s: u32_to_failure!(call!(in_square_brackets)) >>
        ts: u32_to_failure!(expr_res!(date(s))) >>
        (Timestamp::InactiveDate(ts.1))
    )
);

fn date_with_time_range(s: &str) -> IResult<&str, Timestamp, Error> {
    use macros::GenericError;
    // s = 2018-06-04 Mon 12:00-13:00
    println!("{:?}", s);
    let parts: Vec<_> = s.rsplitn(2, ' ').collect();

    let range = &parts[0];
    let date = &parts[1];

    if !range.contains('-') {
        return Err(::nom::Err::Error(error_position!(
            s,
            ::nom::ErrorKind::Custom(GenericError::from(2).into())
        )));
    }

    let times: Vec<_> = range
        .split('-')
        .map(|time| NaiveTime::parse_from_str(time, "%H:%M"))
        .collect();
    let start_time = times[0].map_err(|_| {
        ::nom::Err::Error(error_position!(
            s,
            ::nom::ErrorKind::Custom(GenericError::from(1).into())
        ))
    })?;
    let end_time = times[1].map_err(|_| {
        ::nom::Err::Error(error_position!(
            s,
            ::nom::ErrorKind::Custom(GenericError::from(1).into())
        ))
    })?;
    let date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&date, "%Y-%m-%d %a"))
    {
        Ok(d) => d,
        Err(_) => {
            return Err(::nom::Err::Error(error_position!(
                s,
                ::nom::ErrorKind::Custom(GenericError::from(2).into())
            )))
        }
    };

    Ok((
        "",
        Timestamp::TimeRange {
            date,
            start_time,
            end_time,
        },
    ))
}

named!(active_time_range<&str, Timestamp, Error>,
    do_parse!(
        s: u32_to_failure!(in_angle_brackets) >>
        tr: u32_to_failure!(expr_res!(date_with_time_range(s))) >>
        (tr.1)
    )
);

named!(timestamp<&str, Timestamp, Error>,
       alt!(call!(active_date) | call!(active_datetime) | call!(active_time_range) | call!(inactive_date) | call!(inactive_datetime)));

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_in_angle_brackets() {
        assert_eq!(in_angle_brackets("<2018-05-13>"), Ok(("", "2018-05-13")));
        assert_eq!(in_angle_brackets("<2018>-05-13>"), Ok(("-05-13>", "2018")));
        assert!(in_angle_brackets("fdsajhaslkjdhf").is_err());
    }

    #[test]
    fn test_in_square_brackets() {
        assert_eq!(in_square_brackets("[2018-05-13]"), Ok(("", "2018-05-13")));
        assert_eq!(in_square_brackets("[2018]-05-13]"), Ok(("-05-13]", "2018")));
        assert!(in_square_brackets("fdsajhaslkjdhf").is_err());
    }

    #[test]
    fn test_parse_date() {
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
    fn test_parse_datetime() {
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

    //    #[test]
    //    fn test_parse_active_timestamp() {
    //        assert_eq!(
    //            "<2018-06-22 Fri>".parse(),
    //            Ok(OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 6, 22)))
    //        );
    //        assert_eq!(
    //            "<2018-06-22>".parse(),
    //            Ok(OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 6, 22)))
    //        );
    //        assert_eq!(
    //            "<2018-06-22 Fri 14:00>".parse(),
    //            Ok(OrgTimestamp::ActiveDateTime(naive_date_time(
    //                2018, 6, 22, 14, 0, 0
    //            )))
    //        );
    //        assert_eq!(
    //            "<2018-06-22 14:00>".parse(),
    //            Ok(OrgTimestamp::ActiveDateTime(naive_date_time(
    //                2018, 6, 22, 14, 0, 0
    //            )))
    //        );
    //        assert_eq!(
    //            "<2018-04-12 13:00-14:30>".parse(),
    //            Ok(OrgTimestamp::TimeRange {
    //                date: NaiveDate::from_ymd(2018, 4, 12),
    //                start_time: NaiveTime::from_hms(13, 0, 0),
    //                end_time: NaiveTime::from_hms(14, 30, 0)
    //            })
    //        );
    //    }
    //
    //    #[test]
    //    fn test_parse_inactive_timestamp() {
    //        assert_eq!(
    //            "[2018-06-22 Fri]".parse(),
    //            Ok(OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 6, 22)))
    //        );
    //        assert_eq!(
    //            "[2018-06-22]".parse(),
    //            Ok(OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 6, 22)))
    //        );
    //        assert_eq!(
    //            "[2018-06-22 Fri 14:00]".parse(),
    //            Ok(OrgTimestamp::InactiveDateTime(naive_date_time(
    //                2018, 6, 22, 14, 0, 0
    //            )))
    //        );
    //        assert_eq!(
    //            "[2018-06-22 14:00]".parse(),
    //            Ok(OrgTimestamp::InactiveDateTime(naive_date_time(
    //                2018, 6, 22, 14, 0, 0
    //            )))
    //        );
    //    }
    //
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
