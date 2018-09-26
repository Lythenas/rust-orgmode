//! This is a library for working with [org files](https://orgmode.org/).
//!
//! Org files are on the surface like markdown files with different syntax. However emacs org mode
//! supports a lot more features than simple markdown. In addition to simply being a markup and
//! outlining language
//! > Org mode is for keeping notes, maintaining TODO
//! > lists, planning projects, and authoring documents with a fast and effective plain-text system.
//! >
//! > -- [org mode](https://orgmode.org/)
//!
//! This library is aimed to support most org mode features. But org mode is very comprehensive.
//!
//! Currently only parsing of the major outline and timestamp is supported.
#![feature(plugin)]
#![feature(min_const_fn)]
#![feature(pattern)]
#![feature(const_vec_new)]
#![plugin(phf_macros)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate rust_orgmode_derive;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate phf;

pub mod entities;
pub mod types;
#[macro_use]
pub mod macros;
#[macro_use]
mod enum_from_str;
mod parse;

use failure::Error;
use std::str::FromStr;

use chrono::prelude::*;

/// Represents an org file.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct OrgFile {
    properties: Vec<Keyword>,
    preface: Section,
    headlines: Vec<Headline>,
}

impl OrgFile {
    pub fn new<T, U, V>(properties: T, preface: U, headlines: V) -> Self
    where
        T: Into<Vec<Keyword>>,
        U: Into<Section>,
        V: Into<Vec<Headline>>,
    {
        OrgFile {
            preface: preface.into(),
            properties: properties.into(),
            headlines: headlines.into(),
        }
    }
}

impl FromStr for OrgFile {
    type Err = OrgParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::types::CompleteStr;
        use nom::ErrorKind;
        ::parse::file(CompleteStr(s))
            .or_else(|err| {
                match err.into_error_kind() {
                    // TODO convert to useful error
                    ErrorKind::Custom(e) => Err(OrgParseError::Custom(e)),
                    _ => unimplemented!(),
                }
            }).and_then(|(s, ts)| {
                if s == CompleteStr("") {
                    Ok(ts)
                } else {
                    Err(OrgParseError::TooMuchInput(s.to_string()))
                }
            })
    }
}

/// Represents a keyword in an org file.
///
/// TODO if keyword belongs to document properties it's value can contain objects.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Keyword {
    key: String,
    value: String,
}

impl Keyword {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Keyword {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug)]
pub enum OrgParseError {
    TooMuchInput(String),
    Custom(Error),
}

/// Represents a headline in an org file.
///
/// `STARS KEYWORD PRIORITY TITLE TAGS`
///
/// The stars represent the level.
///
/// The keyword is associated with a specific state.
///
/// The priority is of format `[#A]`. Where `A` is a letter from `A` to `Z`.
///
/// The title is arbitrary (but no newlines). If the title starts with `COMMENT`
/// the headline will be considered as commented.
///
/// The tags are in the following format: `:tag1:tag2:` and can contain any
/// alpha-numeric character, underscore, at sign, hash sign or percent sign.
///
/// A headline can contain directly one section and multiple sub headlines
/// that are (at least?) one level deeper.
#[derive(Debug, PartialEq, Eq)]
pub struct Headline {
    level: usize,
    keyword: Option<State>,
    priority: Option<Priority>,
    title: String,
    commented: bool,
    tags: Vec<String>,
    planning: Planning,
    property_drawer: PropertyDrawer,
    section: Option<Section>,
    sub_headlines: Vec<Headline>,
    affiliated_keywords: Vec<AffiliatedKeyword>,
}

impl Headline {
    pub fn new(level: usize, title: impl Into<String>) -> Self {
        let title = title.into();
        Headline {
            level,
            keyword: None,
            priority: None,
            commented: title.starts_with("COMMENT"),
            title,
            tags: Vec::new(),
            planning: Planning::default(),
            property_drawer: PropertyDrawer::default(),
            section: None,
            sub_headlines: Vec::new(),
            affiliated_keywords: Vec::new(),
        }
    }
    pub fn and_keyword(self, keyword: State) -> Self {
        self.and_opt_keyword(Some(keyword))
    }
    pub fn and_opt_keyword(self, keyword: Option<State>) -> Self {
        Headline { keyword, ..self }
    }
    pub fn and_priority(self, priority: Priority) -> Self {
        self.and_opt_priority(Some(priority))
    }
    pub fn and_opt_priority(self, priority: Option<Priority>) -> Self {
        Headline { priority, ..self }
    }
    pub fn and_tags(self, tags: Vec<String>) -> Self {
        Headline { tags, ..self }
    }
    pub fn and_opt_tags(self, tags: Option<Vec<String>>) -> Self {
        self.and_tags(tags.unwrap_or_default())
    }
    pub fn and_planning(self, planning: Planning) -> Self {
        Headline { planning, ..self }
    }
    pub fn and_property_drawer(self, property_drawer: PropertyDrawer) -> Self {
        Headline {
            property_drawer,
            ..self
        }
    }
    pub fn and_section(self, section: impl Into<Section>) -> Self {
        self.and_opt_section(Some(section.into()))
    }
    pub fn and_opt_section(self, section: Option<Section>) -> Self {
        Headline { section, ..self }
    }
    pub fn and_sub_headlines(self, sub_headlines: Vec<Headline>) -> Self {
        Headline {
            sub_headlines,
            ..self
        }
    }
    pub fn and_affiliated_keywords(self, affiliated_keywords: Vec<AffiliatedKeyword>) -> Self {
        Headline {
            affiliated_keywords,
            ..self
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn sub_headlines_mut(&mut self) -> &mut Vec<Headline> {
        &mut self.sub_headlines
    }
}

/// Property drawer contains properties for a [`Headline`].
#[derive(Debug, PartialEq, Eq, Default)]
pub struct PropertyDrawer(Vec<Property>);

impl PropertyDrawer {
    pub fn new(vec: Vec<Property>) -> Self {
        PropertyDrawer(vec)
    }
    pub fn empty() -> Self {
        PropertyDrawer::new(Vec::new())
    }
}

/// A property of a [`PropertyDrawer`].
#[derive(Debug, PartialEq, Eq)]
pub enum Property {
    KeyValue(String, String),
    KeyPlusValue(String, String),
    Key(String),
    KeyPlus(String),
}

/// Planning information for a headline.
///
/// Contains the deadline, scheduled and closed [`Timestamp`]s.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Planning {
    deadline: Option<Timestamp>,
    scheduled: Option<Timestamp>,
    closed: Option<Timestamp>,
}

impl Planning {
    pub fn new() -> Self {
        Planning {
            deadline: None,
            scheduled: None,
            closed: None,
        }
    }
    pub fn and_deadline(self, deadline: Timestamp) -> Self {
        self.and_opt_deadline(Some(deadline))
    }
    pub fn and_opt_deadline(self, deadline: Option<Timestamp>) -> Self {
        Planning { deadline, ..self }
    }
    pub fn and_scheduled(self, scheduled: Timestamp) -> Self {
        self.and_opt_scheduled(Some(scheduled))
    }
    pub fn and_opt_scheduled(self, scheduled: Option<Timestamp>) -> Self {
        Planning { scheduled, ..self }
    }
    pub fn and_closed(self, closed: Timestamp) -> Self {
        self.and_opt_closed(Some(closed))
    }
    pub fn and_opt_closed(self, closed: Option<Timestamp>) -> Self {
        Planning { closed, ..self }
    }
}

/// This represents a section in a org file.
///
/// A section is the text, tables, etc. after a headline but before the next headline.
///
/// **Note:** Currently this is only a [`String`] but in the future it will contain more fine
/// grained parsed elements.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Section(String);

impl Section {
    fn new(s: impl Into<String>) -> Self {
        Section(s.into())
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> From<T> for Section
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        Section::new(s)
    }
}

/// The state of a [`Headline`].
///
/// The enum variants accept an additional string because the actual keyword
/// signaling the state of the [`Headline`] can be configured (defaults are
/// listed below).
///
/// **Note**: Currently only the default keywords are accepted.
#[derive(Debug, PartialEq, Eq)]
pub enum State {
    /// Default keywords: `TODO` and `NEXT`
    Todo(String),
    /// Default keyword: `DONE`
    Done(String),
}

enum_from_str!(
    #[doc="Represents a priority of a [`Headline`]."]
    Priority => A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);

/// Represents an attribute for various other elements (currently only [`Headline`]).
///
/// Inlinetasks, items, planning, clocks, node properties and table rows can't have attributes.
///
/// The affilicated keywords are placed just above the item and follow one of the following
/// patterns:
///
/// * `#+KEY: VALUE`
/// * `#+KEY[OPTIONAL]: VALUE`
/// * `#+ATTR_BACKEND: VALUE`
///
/// `KEY` is either `CAPTION`, `HEADER`, `NAME`, `PLOT` or `RESULTS`.
///
/// `BACKEND` is a string constituted of alpha-numeric characters, hyphens or underscores.
///
/// `OPTIONAL` and `VALUE` can contain any character but a new line.
/// Only `CAPTION` and `RESULTS` keywords can have an optional value.
///
/// An affiliated keyword can appear more than once if `KEY` is either `CAPTION` or `HEADER`
/// or if its pattern is `#+ATTR_BACKEND: VALUE`.
///
/// `CAPTION`, `AUTHOR`, `DATE` and `TITLE` keywords can contain objects in their value
/// and their optional value, if applicable. (Objects are not implemented yet)
#[derive(Debug, PartialEq, Eq)]
pub struct AffiliatedKeyword {
    kind: AffiliatedKeywordKind,
    value: AffiliatedKeywordValue,
}

impl AffiliatedKeyword {
    pub fn new(kind: AffiliatedKeywordKind, value: AffiliatedKeywordValue) -> Self {
        AffiliatedKeyword { kind, value }
    }
}

/// This represents the kind of a [`AffiliatedKeyword`].
#[derive(Debug, PartialEq, Eq)]
pub enum AffiliatedKeywordKind {
    /// The caption kind can have a optional value (See: [`AffiliatedKeyword`]).
    Caption(Option<AffiliatedKeywordValue>),
    Header,
    Name,
    Plot,
    /// The results kind can have an optional value (See: [`AffiliatedKeyword`]).
    Results(Option<AffiliatedKeywordValue>),
    /// The attr kind has a backend that is an arbitrary string.
    Attr(String),
}

/// This is a value of an [`AffiliatedKeyword`].
///
/// Currently this only wraps a string. This will contain objects in the future for
/// caption, author, date and title (optional) value.
#[derive(Debug, PartialEq, Eq)]
pub struct AffiliatedKeywordValue(String);

impl AffiliatedKeywordValue {
    pub fn new(value: impl Into<String>) -> Self {
        AffiliatedKeywordValue(value.into())
    }
}

/// Represents a timestamp in an org file. The variants are the same
/// mentioned in the [Org Syntax][org].
///
/// The diary variant is not implemented.
///
/// [org]: https://orgmode.org/worg/dev/org-syntax.html#Timestamp
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
    /// # extern crate rust_orgmode;
    /// # use chrono::NaiveDate;
    /// # use rust_orgmode::Timestamp;
    /// # use rust_orgmode::TimestampData;
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

#[derive(Debug)]
pub enum TimestampParseError {
    TooMuchInput(String),
    Custom(Error),
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_to_z_is_parseable_to_priority() {
        use std::char;

        for i in 'A' as u32..('Z' as u32 + 1) {
            let prio = &char::from_u32(i).unwrap().to_string().parse::<Priority>();
            assert!(prio.is_ok());
        }
    }

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
