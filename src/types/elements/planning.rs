use super::*;

/// A planning element.
///
/// # Semantics
///
/// Contains the deadline, scheduled and closed timestamps for a headline. All are optional.
///
/// # Syntax
///
/// Planning lines are context-free.
///
/// ```text
/// KEYWORD: TIMESTAMP
/// ```
///
/// `KEYWORD` is one of `DEADLINE`, `SCHEDULED` or `CLOSED`. Planning can be repeated but one
/// keywords can only be used once. The order doesn't matter.
///
/// `TIMESTAMP` is a [`objects::Timestamp`].
///
/// Consecutive planning items are aggregated into one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Planning {
    pub closed: Option<objects::Timestamp>,
    pub deadline: Option<objects::Timestamp>,
    pub scheduled: Option<objects::Timestamp>,
}
