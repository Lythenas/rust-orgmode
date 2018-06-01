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
       alt!(dbg_dmp!(call!(datetime_with_weekday)) | dbg_dmp!(call!(datetime_without_weekday))));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_datetime() {
        assert_eq!(
            active_datetime("<2018-05-13 14:44>").ok(),
            Some((
                "",
                Timestamp::ActiveDateTime(NaiveDate::from_ymd(2018, 05, 13).and_hms(14, 44, 0))
            ))
        );
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
            date("2018-05-13"),
            Ok(("", NaiveDate::from_ymd(2018, 05, 13)))
        );
        assert_eq!(
            date("2018-05-13 Sun"),
            Ok(("", NaiveDate::from_ymd(2018, 05, 13)))
        );
        assert!(date("adadasd").is_err());
    }

    #[test]
    fn test_parse_datetime() {
        //assert_eq!(
        //    datetime("2018-05-13 12:40"),
        //    Ok(("", NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)))
        //);
        //assert_eq!(
        //    datetime("2018-05-13 Sun 12:40"),
        //    Ok(("", NaiveDate::from_ymd(2018, 05, 13).and_hms(12, 40, 0)))
        //);
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
