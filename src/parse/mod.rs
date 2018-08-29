use failure::Error;
use nom::types::CompleteStr;
use nom::IResult;

// See toc of https://orgmode.org/worg/dev/org-syntax.html#Headlines_and_Sections
mod file;
mod headlines;
mod affiliated_keywords;
//mod greater_elements;
//mod elements;
//mod objects;

mod timestamp;

pub use self::file::file;
pub use self::timestamp::timestamp;
pub use self::headlines::{headline, section};
pub use self::affiliated_keywords::{single_affiliated_keyword, affiliated_keywords};

pub type OrgInput<'a> = CompleteStr<'a>;
pub type OrgResult<'a, T> = IResult<OrgInput<'a>, T, Error>;

