use chrono::prelude::*;
use failure::Error;
use std::str::FromStr;

/// Represents the action that is taken when you mark a task with
/// a repeater as `DONE`.
///
/// Contains the amount of time to use when repeating and the strategy
/// to use when applying the repeater (see [`RepeatStrategy`]).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Repeater {
    period: TimePeriod,
    strategy: RepeatStrategy,
}

impl Repeater {
    /// Constructs a new `Repeater` with the specified time period and
    /// repeat strategy.
    pub fn new(period: TimePeriod, strategy: RepeatStrategy) -> Self {
        Repeater { period, strategy }
    }
}

/// The repeat strategies for a [`Repeater`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RepeatStrategy {
    /// Add the repeat duration to the task date once.
    Cumulative,
    /// Add the repeat duration to the task date until the date is in the
    /// future (but at leas once).
    CatchUp,
    /// Add the repeat duration to the current time.
    Restart,
}

/// Represents a warning delay for a [`Timestamp`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WarningDelay {
    delay: TimePeriod,
    strategy: WarningStrategy,
}

impl WarningDelay {
    pub fn new(delay: TimePeriod, strategy: WarningStrategy) -> Self {
        WarningDelay { delay, strategy }
    }
}

/// The warning strategy for a [`WarningDelay`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WarningStrategy {
    /// Warns for all (repeated) date. Represented as `-` in the org file.
    All,
    /// Warns only for the first date. Represented as `--` in the org file.
    First,
}

/// Represents a amount of time.
///
/// Used e.g. as the warning period and in repeater.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimePeriod {
    value: u32,
    unit: TimeUnit,
}

impl TimePeriod {
    /// Constructs a new `TimePeriod` with the specified unit and amount.
    pub fn new(value: u32, unit: TimeUnit) -> Self {
        Self { value, unit }
    }
}

/// Represents the unit of time used for `Repeater` and `TimePeriod`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
}

/// Convenience trait implemented on `u32` to easily convert to a `TimePeriod`.
pub trait AsTimePeriod {
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Year`.
    fn year(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Month`.
    fn month(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Week`.
    fn week(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Day`.
    fn day(self) -> TimePeriod;
    /// Convert self to a `TimePeriod` wit unit `TimeUnit::Hour`.
    fn hour(self) -> TimePeriod;
}

impl AsTimePeriod for u32 {
    fn year(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Year)
    }
    fn month(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Month)
    }
    fn week(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Week)
    }
    fn day(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Day)
    }
    fn hour(self) -> TimePeriod {
        TimePeriod::new(self, TimeUnit::Hour)
    }
}

/// Represents a timestamp range. This is used for [`Timestamp::ActiveRange`]
/// and [`Timestamp::InactiveRange`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TimestampRange {
    /// `<DATE TIME-TIME REPEATER-OR-DELAY>` or
    /// `[DATE TIME-TIME REPEATER-OR-DELAY]`
    TimeRange(TimestampDataWithTime, Time),
    /// `<DATE TIME REPEATER-OR-DELAY>--<DATE TIME REPEATER-OR-DELAY>` or
    /// `[DATE TIME REPEATER-OR-DELAY]--[DATE TIME REPEATER-OR-DELAY]`
    DateRange(TimestampData, TimestampData),
}

/// Internal data of a *normal* timestamp with optional [`Time`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimestampData {
    date: Date,
    time: Option<Time>,
    repeater: Option<Repeater>,
    warning_delay: Option<WarningDelay>,
}

impl TimestampData {
    pub fn new(date: impl Into<Date>) -> Self {
        Self {
            date: date.into(),
            time: None,
            repeater: None,
            warning_delay: None,
        }
    }
    pub fn with_time(date: impl Into<Date>, time: impl Into<Time>) -> Self {
        Self {
            date: date.into(),
            time: Some(time.into()),
            repeater: None,
            warning_delay: None,
        }
    }
    pub fn and_time(self, time: impl Into<Time>) -> Self {
        self.and_opt_time(Some(time.into()))
    }
    pub fn and_opt_time(self, time: Option<impl Into<Time>>) -> Self {
        Self {
            time: time.map(Into::into),
            ..self
        }
    }
    pub fn and_repeater(self, repeater: Repeater) -> Self {
        self.and_opt_repeater(Some(repeater))
    }
    pub fn and_opt_repeater(self, repeater: Option<Repeater>) -> Self {
        Self { repeater, ..self }
    }
    pub fn and_warning_delay(self, warning_delay: WarningDelay) -> Self {
        self.and_opt_warning_delay(Some(warning_delay))
    }
    pub fn and_opt_warning_delay(self, warning_delay: Option<WarningDelay>) -> Self {
        Self {
            warning_delay,
            ..self
        }
    }

    pub fn get_date(&self) -> &Date {
        &self.date
    }
    pub fn get_time(&self) -> &Option<Time> {
        &self.time
    }
    pub fn get_repeater(&self) -> &Option<Repeater> {
        &self.repeater
    }
    pub fn get_warning_delay(&self) -> &Option<WarningDelay> {
        &self.warning_delay
    }
}

/// Internal data of a timestamp with required [`Time`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimestampDataWithTime {
    date: Date,
    time: Time,
    repeater: Option<Repeater>,
    warning_delay: Option<WarningDelay>,
}

impl TimestampDataWithTime {
    pub fn new(date: impl Into<Date>, time: impl Into<Time>) -> Self {
        TimestampDataWithTime {
            date: date.into(),
            time: time.into(),
            repeater: None,
            warning_delay: None,
        }
    }

    pub fn and_repeater(self, repeater: Repeater) -> Self {
        self.and_opt_repeater(Some(repeater))
    }
    pub fn and_opt_repeater(self, repeater: Option<Repeater>) -> Self {
        Self { repeater, ..self }
    }
    pub fn and_warning_delay(self, warning_delay: WarningDelay) -> Self {
        self.and_opt_warning_delay(Some(warning_delay))
    }
    pub fn and_opt_warning_delay(self, warning_delay: Option<WarningDelay>) -> Self {
        Self {
            warning_delay,
            ..self
        }
    }
}

/// Wrapper for the date of a timestamp.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Date(NaiveDate);

impl Date {
    pub fn new(date: NaiveDate) -> Self {
        Date(date)
    }
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Date {
        Date(date)
    }
}

/// Wrapper for the time of a timestamp.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Time(NaiveTime);

impl Time {
    pub fn new(time: NaiveTime) -> Self {
        Time(time)
    }
}

impl From<NaiveTime> for Time {
    fn from(time: NaiveTime) -> Time {
        Time(time)
    }
}

/// Represents a timestamp in an org file. The variants are the same
/// mentioned in [https://orgmode.org/worg/dev/org-syntax.html#Timestamp].
///
/// The diary variant is not implemented.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Timestamp {
    //Diary,
    /// `<DATE TIME REPEATER-OR-DELAY>`
    Active(TimestampData),
    /// `[DATE TIME REPEATER-OR-DELAY]`
    Inactive(TimestampData),
    /// `<DATE TIME REPEATER-OR-DELAY>--<DATE TIME REPEATER-OR-DELAY>` or
    /// `<DATE TIME-TIME REPEATER-OR-DELAY>`
    ActiveRange(TimestampRange),
    /// `[DATE TIME REPEATER-OR-DELAY]--[DATE TIME REPEATER-OR-DELAY]` or
    /// `[DATE TIME-TIME REPEATER-OR-DELAY]`
    InactiveRange(TimestampRange),
}

impl Timestamp {
    /// Returns `true` if the org timestamp is active.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate orgmode;
    /// # use chrono::NaiveDate;
    /// # use orgmode::Timestamp;
    /// # use orgmode::TimestampData;
    /// #
    /// let ts = Timestamp::Active(
    ///     TimestampData::new(NaiveDate::from_ymd(2018, 04, 28))
    /// );
    /// assert_eq!(ts.is_active(), true);
    ///
    /// let ts = Timestamp::Inactive(
    ///     TimestampData::new(NaiveDate::from_ymd(2018, 04, 28))
    /// );
    /// assert_eq!(ts.is_active(), false);
    /// ```
    pub fn is_active(&self) -> bool {
        match self {
            Timestamp::Active(_) | Timestamp::ActiveRange(_) => true,
            _ => false,
        }
    }
}

impl FromStr for Timestamp {
    type Err = TimestampParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::types::CompleteStr;
        use nom::ErrorKind;
        ::parse::timestamp(CompleteStr(s))
            .or_else(|err| {
                match err.into_error_kind() {
                    // TODO convert to useful error
                    ErrorKind::Custom(e) => Err(TimestampParseError::Custom(e)),
                    _ => unimplemented!(),
                }
            }).and_then(|(s, ts)| {
                if s == CompleteStr("") {
                    Ok(ts)
                } else {
                    Err(TimestampParseError::TooMuchInput(s.to_string()))
                }
            })
    }
}

#[derive(Debug)]
pub enum TimestampParseError {
    TooMuchInput(String),
    Custom(Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod time_period {
        use super::*;

        #[test]
        fn test_year() {
            assert_eq!(44.year(), TimePeriod::new(44, TimeUnit::Year));
        }

        #[test]
        fn test_month() {
            assert_eq!(44.month(), TimePeriod::new(44, TimeUnit::Month));
        }

        #[test]
        fn test_day() {
            assert_eq!(44.day(), TimePeriod::new(44, TimeUnit::Day));
        }

        #[test]
        fn test_hour() {
            assert_eq!(44.hour(), TimePeriod::new(44, TimeUnit::Hour));
        }
    }

    mod timestamp {
        use super::*;

        #[test]
        #[ignore]
        fn test_from_str() {
            assert_eq!(
                "<2018-06-13 21:22>".parse().ok(),
                Some(Timestamp::Active(TimestampData::with_time(
                    NaiveDate::from_ymd(2018, 06, 13),
                    NaiveTime::from_hms(21, 22, 0)
                )))
            );
        }
    }
}
