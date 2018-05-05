use regex::Captures;
use std::str::FromStr;

/// Helper trait for [`regex::Captures`] that automatically parses the capture group to the needed
/// type.
pub trait ParseCaptures {
    /// Returns the match associated with the capture group at index `i` parsed to the needed type.
    /// If `i` does not correspond to a capture group, or if the capture group did not participate
    /// in the match, or if `parse` returns an error, then `None` is returned.
    fn parse<T: FromStr>(&self, i: usize) -> Option<T>;

    /// Returns the match for the capture group named `name`. If `name` isn't a valid capture group
    /// or didn't match anything, or if `parse` returns an error, then None is returned.
    fn parse_name<T: FromStr>(&self, name: &str) -> Option<T>;
}

impl<'t> ParseCaptures for Captures<'t> {
    fn parse<T: FromStr>(&self, i: usize) -> Option<T> {
        self.get(i)?.as_str().parse().ok()
    }

    fn parse_name<T: FromStr>(&self, name: &str) -> Option<T> {
        self.name(&name)?.as_str().parse().ok()
    }
}
