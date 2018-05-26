use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use regex::Captures;
use regex::Regex;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to generate a NaiveDateTime easily
    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> NaiveDateTime {
        NaiveDate::from_ymd(year, month, day).and_time(NaiveTime::from_hms(hour, min, sec))
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
