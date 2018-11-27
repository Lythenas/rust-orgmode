//! Contains the types and traits needed for parsing.

#[allow(unused_imports)]
use pest::Parser;

#[derive(Parser)]
#[grammar = "orgmode.pest"]
pub struct OrgModeParser;

#[cfg(test)]
mod tests {
    use super::*;
}
