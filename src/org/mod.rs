use std::str::FromStr;
use std::collections::HashMap;

use chrono::prelude::*;
use chrono::Duration;

/// An error which can be returned when parsing an [`OrgFile`] or any of its containing parts.
#[derive(Debug, PartialEq, Eq)]
pub enum OrgParseError {
    Unknown,
    Partial(Box<OrgParseError>, OrgFile),
    Syntax(String),
}

/// Represents a org file.
///
/// # Usage
///
/// Create a OrgFile from a string:
///
/// ```ignore
/// use std::fs::File;
/// use std::io::prelude::*;
///
/// let mut file = File::open("notebook.org")?;
/// let mut contents = String::new();
/// file.read_to_string(&mut contents)?;
/// let orgfile : OrgFile = content.parse().unwrap();
/// ```
///
/// Create a OrgFile manually and save it to a string. This example creates an empty OrgFile with a
/// preface. Add items with [`OrgFile::add_node`].
///
/// ```ignore
/// let mut orgfile = OrgFile::new();
/// orgfile.set_preface("Example notebook");
///
/// let mut file = File::create("notebook.org")?;
/// file.write_all(orgfile.to_string())?; 
/// ```
#[derive(Debug, PartialEq, Eq, Default)]
pub struct OrgFile {
    preface: String,
    properties: HashMap<String, String>,
    nodes: Vec<OrgNode>,
}

impl OrgFile {
    fn new() -> Self {
        Self::default()
    }
}

impl FromStr for OrgFile {
    type Err = OrgParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO parse
        Err(OrgParseError::Unknown)
    }
}

/// Represents one *node* in the org file. A node is a headline (a line starting with one or more `*`).
///
/// This node can contain many more nodes that are sub-headlines of this one.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct OrgNode {
    title: String,
    state: OrgState,
    priority: Priority,
    tags: Vec<String>,
    scheduled: Option<OrgDate>,
    deadline: Option<OrgDate>,
    closed: Option<OrgDate>,
    properties: OrgProperties,
    content: OrgContent,
    level: u8,
    commented: bool,
    nodes: Vec<OrgNode>,
}

impl FromStr for OrgNode {
    type Err = OrgParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO parse
        Err(OrgParseError::Unknown)
    }
}

/// Contains all the string accepted as [`OrgState::Todo`].
/// 
/// TODO make this configurable
static ORG_TODO_STATES: [&'static str; 2] = ["TODO", "NEXT"];

/// Contains all the string accepted as [`OrgState::Done`].
///
/// TODO make this configurable
static ORG_DONE_STATES: [&'static str; 1] = ["DONE"];

/// The state of a [`OrgNode`]. Can be eighter `Todo` or `Done`. The enum variants accept an
/// additional string because the actual keyword signaling the state of the `OrgNode` can be
/// anything.
///
/// Currently only keywords specified in [`ORG_TODO_STATES`] are parsed as `Todo`. All other
/// keywords is parsed as `Done`. No keyword present a.k.a an empty string will be parsed as
/// `None`.
#[derive(Debug, PartialEq, Eq)]
pub enum OrgState {
    Todo(String),
    Done(String),
    None,
}

impl Default for OrgState {
    fn default() -> Self {
        OrgState::Done(String::new())
    }
}

impl FromStr for OrgState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if (ORG_TODO_STATES.contains(&s)) {
            Ok(OrgState::Todo(s.to_string()))
        } else if (ORG_DONE_STATES.contains(&s)) {
            Ok(OrgState::Done(s.to_string()))
        } else {
            Ok(OrgState::None)
        }
    }
}

/// Represents a date in a org file. The date can eighter be just a date (without time), a date
/// with time, an interval (containing a start and end date with time) or a repeatable date with
/// time.
///
/// TODO rework this (possibly as struct)
#[derive(Debug, PartialEq, Eq)]
pub enum OrgDate {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Interval {
        start: NaiveDateTime,
        end: NaiveDateTime
    },
    Repeat(NaiveDateTime, Duration),
}

impl Default for OrgDate {
    fn default() -> Self {
        OrgDate::Date(Utc::today().naive_utc())
    }
}

impl FromStr for OrgDate {
    type Err = ::chrono::format::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO parse
        Ok(OrgDate::Date(Utc::today().naive_utc()))
    }
}

pub type OrgProperties = HashMap<String, String>;

/// Represents the content (section) for one headline.
///
/// TODO make this more detailed than just a string
#[derive(Debug, PartialEq, Eq, Default)]
pub struct OrgContent {
    value: String,
}

impl FromStr for OrgContent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OrgContent {
            value: s.to_string()
        })
    }
}

/// Creates an enum with the given name and empty variants.
/// Automatically implements FromStr to parse it easily and Display to print it easily.
/// Also derives Clone, Debug, PartialEq, Eq and Hash for this enum.
macro_rules! parsable_simple_enum {
    ($name:ident, $( $x:ident ),+ ) => {
        use std::fmt;
        use std::str::FromStr;
        
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $x,  
            )+
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($x) => Ok($name::$x),
                    )+
                    _ => Err(())
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self {
                    $(
                        $name::$x => write!(f, stringify!($x)),
                    )+
                }
            }
        }
    };
}

parsable_simple_enum!(Priority, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

impl Default for Priority {
    fn default() -> Self {
        Priority::A
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_org_state() {
        assert_eq!(Ok(OrgState::Todo("TODO".to_string())), "TODO".parse());
        assert_eq!(Ok(OrgState::Todo("NEXT".to_string())), "NEXT".parse());
        assert_eq!(Ok(OrgState::Done("DONE".to_string())), "DONE".parse());
        assert_eq!(Ok(OrgState::None), "".parse());
    }
    
    #[test]
    fn test_parsable_simple_enum_generation() {
        parsable_simple_enum!(TestEnum, One, Two, Three);
        
        let one = TestEnum::from_str("One").unwrap();
        assert_eq!(one, TestEnum::One);
        assert_eq!(format!("{}", one), "One");

        let two = TestEnum::from_str("Two").unwrap();
        assert_eq!(two, TestEnum::Two);
        assert_eq!(format!("{}", two), "Two");

        let three = TestEnum::from_str("Three").unwrap();
        assert_eq!(three, TestEnum::Three);
        assert_eq!(format!("{}", three), "Three");
    }

    #[test]
    fn test_a_to_z_is_parsable_to_priority() {
        use std::char;

        for i in 'A' as u32..('Z' as u32 + 1) {
            let prio = Priority::from_str(&char::from_u32(i).unwrap().to_string());
            assert!(prio.is_ok());
        }
    }



}
