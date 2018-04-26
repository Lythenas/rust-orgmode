use std::str::FromStr;
use std::collections::HashMap;

use chrono::prelude::*;

mod helpers;
pub use org::helpers::*;

pub struct OrgFile {
    preface: String,
    title: String,
    nodes: Vec<OrgNode>,
}

pub struct OrgNode {
    title: String,
    tags: Vec<String>,
    state: OrgState,
    priority: String,
    scheduled: OrgDate,
    deadline: OrgDate,
    closed: OrgDate,
    properties: OrgProperties,
    content: OrgContent,
    level: u8,
    nodes: Vec<OrgNode>,
}

