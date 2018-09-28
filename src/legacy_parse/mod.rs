//! Contains all the parsers for org files and parts of it.
#![allow(clippy)]

use failure::Error;
use nom::types::CompleteStr;
use nom::IResult;

mod timestamp;

pub use self::timestamp::timestamp;

pub type OrgInput<'a> = CompleteStr<'a>;
pub type OrgResult<'a, T> = IResult<OrgInput<'a>, T, Error>;
