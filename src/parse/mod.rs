//! Contains all the parsers for org files and parts of it.

use failure::Error;
use nom::types::CompleteStr;
use nom::IResult;

// See toc of https://orgmode.org/worg/dev/org-syntax.html#Headlines_and_Sections
mod affiliated_keywords;
mod file;
mod headline;
//mod greater_elements;
//mod elements;
//mod objects;

mod timestamp;

pub use self::affiliated_keywords::{affiliated_keywords, single_affiliated_keyword};
pub use self::file::file;
pub use self::headline::{headline, section};
pub use self::timestamp::timestamp;

pub type OrgInput<'a> = CompleteStr<'a>;
pub type OrgResult<'a, T> = IResult<OrgInput<'a>, T, Error>;
