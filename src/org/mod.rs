use std::str::FromStr;
use std::collections::HashMap;

use chrono::prelude::*;

mod helpers;
pub use org::helpers::*;

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
        // TODO
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
    scheduled: OrgDate,
    deadline: OrgDate,
    closed: OrgDate,
    properties: OrgProperties,
    content: OrgContent,
    level: u8,
    commented: bool,
    nodes: Vec<OrgNode>,
}

