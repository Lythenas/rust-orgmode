use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use regex::Captures;
use regex::Regex;

use org::helpers::*;
use org::*;

/// The error type returned by [`OrgTimestamp::from_str`].
#[derive(Debug, PartialEq, Eq)]
pub enum OrgTimestampParseError {
    ParseError,
    NoTimestampFound,
    NotImplemented,
}

type TimestampResult = Result<OrgTimestamp, OrgTimestampParseError>;

/// Represents a date in an org file. See [https://orgmode.org/manual/Timestamps.html].
#[derive(Debug, PartialEq, Eq)]
pub enum OrgTimestamp {
    InactiveDate(NaiveDate),
    InactiveDateTime(NaiveDateTime),
    ActiveDate(NaiveDate),
    ActiveDateTime(NaiveDateTime),
    TimeRange {
        date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
    },
    DateRange {
        start: NaiveDate,
        end: NaiveDate,
    },
    DateTimeRange {
        start: NaiveDateTime,
        end: NaiveDateTime,
    },
    RepeatingDate(NaiveDate, Duration),
    RepeatingDateTime(NaiveDateTime, Duration),
}

impl OrgTimestamp {
    /// Returns `true` if the org timestamp is active.
    ///
    /// This is the case if it is not one of [`InactiveDate`] or [`InactiveDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::org::OrgTimestamp;
    ///
    /// let x = OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), true);
    ///
    /// let x = OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), false);
    /// ```
    ///
    /// [`InactiveDate`]: #variant.InactiveDate
    /// [`InactiveDateTime`]: #variant.InactiveDateTime
    pub fn is_active(&self) -> bool {
        use org::OrgTimestamp::*;
        match self {
            InactiveDate(_) => false,
            InactiveDateTime(_) => false,
            _ => true,
        }
    }

    /// Returns `true` if the org timestamp is inactive.
    ///
    /// This is the case if it is eighter [`InactiveDate`] or [`InactiveDateTime`].
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::org::OrgTimestamp;
    ///
    /// let x = OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), true);
    ///
    /// let x = OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 04, 28));
    /// assert_eq!(x.is_active(), false);
    /// ```
    ///
    /// [`InactiveDate`]: #variant.InactiveDate
    /// [`InactiveDateTime`]: #variant.InactiveDateTime
    pub fn is_inactive(&self) -> bool {
        !self.is_active()
    }
}

impl Default for OrgTimestamp {
    fn default() -> Self {
        OrgTimestamp::ActiveDateTime(Utc::now().naive_utc())
    }
}

lazy_static! {
    static ref REGEX_TIMESTAMP_RANGE: Regex = Regex::new(r"<(.+)>--<(.+)>").unwrap();
    static ref REGEX_TIMESTAMP_INACTIVE: Regex = Regex::new(r"\[(.+)\]").unwrap();
    static ref REGEX_TIMESTAMP_ACTIVE: Regex = Regex::new(r"<(.+)>").unwrap();
    static ref REGEX_DATE: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})( (?P<weekday>[A-Z][a-z]{2}))?( (?P<rest>.*))?").unwrap();
    static ref REGEX_TIME: Regex = Regex::new(r"(?P<hours>\d{2}):(?P<minutes>\d{2})").unwrap();
    static ref REGEX_TIME_RANGE: Regex = Regex::new(r"(?P<start_hours>\d{2}):(?P<start_minutes>\d{2})-(?P<end_hours>\d{2}):(?P<end_minutes>\d{2})").unwrap();
}

impl FromStr for OrgTimestamp {
    type Err = OrgTimestampParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if let Some(caps) = REGEX_TIMESTAMP_RANGE.captures(trimmed) {
            return parse_range_timestamp(
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str(),
            );
        } else if let Some(caps) = REGEX_TIMESTAMP_INACTIVE.captures(trimmed) {
            return parse_inactive_timestamp(caps.get(1).unwrap().as_str());
        } else if let Some(caps) = REGEX_TIMESTAMP_ACTIVE.captures(trimmed) {
            return parse_active_timestamp(caps.get(1).unwrap().as_str());
        } else {
            return Err(OrgTimestampParseError::NoTimestampFound);
        }
    }
}

/// Helper function that only parses timestamps in the format `<start>--<end>`.
fn parse_range_timestamp(start: &str, end: &str) -> TimestampResult {
    // TODO
    Err(OrgTimestampParseError::NotImplemented)
}

/// Helper function that only parses timestamps in the format `[timestamp]`.
fn parse_inactive_timestamp(timestamp: &str) -> TimestampResult {
    let caps = REGEX_DATE.captures(timestamp);

    let date = try_date(caps.as_ref())?;

    Ok(if let Some(time) = try_time(caps.as_ref()) {
        OrgTimestamp::InactiveDateTime(date.and_time(time))
    } else {
        OrgTimestamp::InactiveDate(date)
    })
}

/// Helper function that only parses timestamps in the format `<timestamp>`.
fn parse_active_timestamp(timestamp: &str) -> TimestampResult {
    let caps = REGEX_DATE.captures(timestamp);

    let date = try_date(caps.as_ref())?;

    Ok(
        if let Some((start_time, end_time)) = try_time_range(caps.as_ref()) {
            OrgTimestamp::TimeRange {
                date,
                start_time,
                end_time,
            }
        } else if let Some(time) = try_time(caps.as_ref()) {
            OrgTimestamp::ActiveDateTime(date.and_time(time))
        } else {
            OrgTimestamp::ActiveDate(date)
        },
    )
}

fn try_date(caps: Option<&Captures>) -> Result<NaiveDate, OrgTimestampParseError> {
    match &caps {
        Some(caps) => get_date_from_captures(&caps).ok_or(OrgTimestampParseError::ParseError),
        None => Err(OrgTimestampParseError::ParseError),
    }
}

fn try_time_range(caps: Option<&Captures>) -> Option<(NaiveTime, NaiveTime)> {
    caps.as_ref()
        .and_then(|caps| REGEX_TIME_RANGE.captures(caps.name("rest")?.as_str()))
        .and_then(|caps| get_time_range_from_captures(&caps))
}

fn try_time(caps: Option<&Captures>) -> Option<NaiveTime> {
    caps.as_ref()
        .and_then(|caps| REGEX_TIME.captures(caps.name("rest")?.as_str()))
        .and_then(|caps| get_time_from_captures(&caps))
}

fn get_date_from_captures<'t>(caps: &Captures<'t>) -> Option<NaiveDate> {
    //println!("Date: {:#?}", caps);
    let year = caps.parse_name("year")?;
    let month = caps.parse_name("month")?;
    let day = caps.parse_name("day")?;

    NaiveDate::from_ymd_opt(year, month, day)
}

fn get_time_from_captures<'t>(caps: &Captures<'t>) -> Option<NaiveTime> {
    //println!("Time: {:#?}", caps);
    let hours = caps.parse_name("hours")?;
    let minutes = caps.parse_name("minutes")?;

    NaiveTime::from_hms_opt(hours, minutes, 0)
}

fn get_time_range_from_captures<'t>(caps: &Captures<'t>) -> Option<(NaiveTime, NaiveTime)> {
    println!("Time: {:#?}", caps);
    let start_hours = caps.parse_name("start_hours")?;
    let start_minutes = caps.parse_name("start_minutes")?;
    let end_hours = caps.parse_name("end_hours")?;
    let end_minutes = caps.parse_name("end_minutes")?;

    let start = NaiveTime::from_hms_opt(start_hours, start_minutes, 0)?;
    let end = NaiveTime::from_hms_opt(end_hours, end_minutes, 0)?;

    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_prefix_chars() {
        assert_eq!(count_prefix_chars("* abc", '*'), 1);
        assert_eq!(count_prefix_chars("*** abc *", '*'), 3);
        assert_eq!(count_prefix_chars("****** abc ** asd *", '*'), 6);
        assert_eq!(count_prefix_chars("* abc ** a", '*'), 1);
        assert_eq!(count_prefix_chars("abs * abc", '*'), 0);
    }

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

    #[test]
    #[ignore]
    fn test_parse_special_node_timestamps() {
        assert_eq!(
            parse_special_node_timestamps("DEADLINE: <2018-02-19 Mon 14:24>"),
            (
                Some(OrgTimestamp::ActiveDateTime(naive_date_time(
                    2018, 2, 19, 14, 24, 0
                ))),
                None,
                None
            )
        );
    }

    #[test]
    fn test_parse_active_timestamp() {
        assert_eq!(
            "<2018-06-22 Fri>".parse(),
            Ok(OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 6, 22)))
        );
        assert_eq!(
            "<2018-06-22>".parse(),
            Ok(OrgTimestamp::ActiveDate(NaiveDate::from_ymd(2018, 6, 22)))
        );
        assert_eq!(
            "<2018-06-22 Fri 14:00>".parse(),
            Ok(OrgTimestamp::ActiveDateTime(naive_date_time(
                2018, 6, 22, 14, 0, 0
            )))
        );
        assert_eq!(
            "<2018-06-22 14:00>".parse(),
            Ok(OrgTimestamp::ActiveDateTime(naive_date_time(
                2018, 6, 22, 14, 0, 0
            )))
        );
        assert_eq!(
            "<2018-04-12 13:00-14:30>".parse(),
            Ok(OrgTimestamp::TimeRange {
                date: NaiveDate::from_ymd(2018, 4, 12),
                start_time: NaiveTime::from_hms(13, 0, 0),
                end_time: NaiveTime::from_hms(14, 30, 0)
            })
        );
    }

    #[test]
    fn test_parse_inactive_timestamp() {
        assert_eq!(
            "[2018-06-22 Fri]".parse(),
            Ok(OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 6, 22)))
        );
        assert_eq!(
            "[2018-06-22]".parse(),
            Ok(OrgTimestamp::InactiveDate(NaiveDate::from_ymd(2018, 6, 22)))
        );
        assert_eq!(
            "[2018-06-22 Fri 14:00]".parse(),
            Ok(OrgTimestamp::InactiveDateTime(naive_date_time(
                2018, 6, 22, 14, 0, 0
            )))
        );
        assert_eq!(
            "[2018-06-22 14:00]".parse(),
            Ok(OrgTimestamp::InactiveDateTime(naive_date_time(
                2018, 6, 22, 14, 0, 0
            )))
        );
    }
}
