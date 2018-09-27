use super::*;

/// A clock element.
///
/// # Sematics
///
/// A clock element is used to track working time. When clocked in the timestamp part is only a
/// date and time. When clocked out the timestamp part is a datetime range. And the duration is
/// the duration of the range.
///
/// The timestamps are usually inactive.
///
/// # Syntax
///
/// ```text
/// CLOCK: TIMESTAMP DURATION
/// ```
///
/// `CLOCK` is the literal word `CLOCK`.
///
/// `TIMESTAMP` and `DURATION` are optional. `TIMESTAMP` is a [`objects::Timestamp`].
///
/// `DURATION` follows the pattern `=> HH:MM` where `HH` is a number containing any number of
/// digits and `MM` is a two digit number.
#[derive(Element, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clock {
    shared_behavior_data: SharedBehaviorData,
    pub timestamp: Option<objects::Timestamp>,
    pub duration: Option<(u64, u8)>,
}

impl Clock {
    pub fn status(&self) -> ClockStatus {
        match self.duration {
            Some(_) => ClockStatus::Closed,
            None => ClockStatus::Running,
        }
    }
}

/// The status of a [`Clock`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClockStatus {
    Running,
    Closed,
}
