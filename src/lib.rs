#![feature(try_from)]
#![feature(trace_macros)]

extern crate chrono;
extern crate regex;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate itertools;

#[macro_use]
pub mod macros;
#[macro_use]
mod enum_from_str;
mod parse;
mod timestamp;

use std::collections::HashMap;

pub use parse::*;
pub use timestamp::*;

/// Represents an org file.
#[derive(Debug, PartialEq, Eq)]
pub struct OrgFile {
    preface: String,
    properties: HashMap<String, String>,
    nodes: Vec<Headline>,
}

/// Represents a headline in an org file.
///
/// `STARS KEYWORD PRIORITY TITLE TAGS`
///
/// The stars represent the level.
///
/// The keyword is associated with a specific state.
///
/// The priority is of format `[#A]`. Where `A` is a letter from `A` to `Z`.
///
/// The title is arbitrary (but no newlines). If the title starts with `COMMENT`
/// the headline will be considered as commented.
///
/// The tags are in the following format: `:tag1:tag2:` and can contain any
/// alpha-numeric character, underscore, at sign, hash sign or percent sign.
///
/// A headline can contain directly one section and multiple sub headlines
/// that are (at least?) one level deeper.
#[derive(Debug, PartialEq, Eq)]
pub struct Headline {
    level: u8,
    keyword: Option<State>,
    priority: Option<Priority>,
    title: String,
    commented: bool,
    tags: Vec<String>,
    planning: Planning,
    property_drawer: PropertyDrawer,
    section: Option<Section>,
    sub_headlines: Vec<Headline>,
}

impl Headline {
    pub fn new(
        level: u8,
        title: impl Into<String>,
    ) -> Self {
        let title = title.into();
        Headline {
            level,
            keyword: None,
            priority: None,
            commented: title.starts_with("COMMENT"),
            title: title,
            tags: Vec::new(),
            planning: Planning::default(),
            property_drawer: PropertyDrawer::default(),
            section: None,
            sub_headlines: Vec::new(),
        }
    }
    pub fn and_keyword(self, keyword: State) -> Self {
        self.and_opt_keyword(Some(keyword))
    }
    pub fn and_opt_keyword(self, keyword: Option<State>) -> Self {
        Headline {
            keyword,
            ..self
        }
    }
    pub fn and_priority(self, priority: Priority) -> Self {
        self.and_opt_priority(Some(priority))
    }
    pub fn and_opt_priority(self, priority: Option<Priority>) -> Self {
        Headline {
            priority,
            ..self
        }
    }
    pub fn and_tags(self, tags: Vec<String>) -> Self {
        Headline {
            tags,
            ..self
        }
    }
    pub fn and_opt_tags(self, tags: Option<Vec<String>>) -> Self {
        self.and_tags(tags.unwrap_or(Vec::new()))
    }
    pub fn and_planning(self, planning: Planning) -> Self {
        Headline {
            planning,
            ..self
        }
    }
    pub fn and_property_drawer(self, property_drawer: PropertyDrawer) -> Self {
        Headline {
            property_drawer,
            ..self
        }
    }
    pub fn and_section(self, section: Section) -> Self {
        self.and_opt_section(Some(section))
    }
    pub fn and_opt_section(self, section: Option<Section>) -> Self {
        Headline {
            section,
            ..self
        }
    }
    pub fn and_sub_headlines(self, sub_headlines: Vec<Headline>) -> Self {
        Headline {
            sub_headlines,
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct PropertyDrawer(Vec<NodeProperty>);

impl PropertyDrawer {
    pub fn new(vec: Vec<NodeProperty>) -> Self {
        PropertyDrawer(vec)
    }
    pub fn empty() -> Self {
        PropertyDrawer::new(Vec::new())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeProperty {
    KeyValue(String, String),
    KeyPlusValue(String, String),
    Key(String),
    KeyPlus(String),
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Planning {
    deadline: Option<Timestamp>,
    scheduled: Option<Timestamp>,
    closed: Option<Timestamp>,
}

impl Planning {
    pub fn new() -> Self {
        Planning {
            deadline: None,
            scheduled: None,
            closed: None,
        }
    }
    pub fn and_deadline(self, deadline: Timestamp) -> Self {
        self.and_opt_deadline(Some(deadline))
    }
    pub fn and_opt_deadline(self, deadline: Option<Timestamp>) -> Self {
        Planning {
            deadline,
            ..self
        }
    }
    pub fn and_scheduled(self, scheduled: Timestamp) -> Self {
        self.and_opt_scheduled(Some(scheduled))
    }
    pub fn and_opt_scheduled(self, scheduled: Option<Timestamp>) -> Self {
        Planning {
            scheduled,
            ..self
        }
    }
    pub fn and_closed(self, closed: Timestamp) -> Self {
        self.and_opt_closed(Some(closed))
    }
    pub fn and_opt_closed(self, closed: Option<Timestamp>) -> Self {
        Planning {
            closed,
            ..self
        }
    }
}

/// This represents a section in a org file. A section is the text, tables, etc. after a headline
/// but before the next headline.
///
/// **Note:** Currently this is only a [`String`] but in the future it will contain more fine
/// grained parsed elements.
#[derive(Debug, PartialEq, Eq)]
pub struct Section(String);

impl Section {
    fn new(s: impl Into<String>) -> Self {
        Section(s.into())
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The state of a [`Headline`]. Can be eighter `Todo` or `Done`. The enum variants accept an
/// additional string because the actual keyword signaling the state of the `OrgNode` can be
/// anything.
///
/// `TODO` and `NEXT` will be parsed as `State::Todo` and `DONE` will be parsed as `State::Done`.
#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Todo(String),
    Done(String),
}

enum_from_str!(
    #[doc="Represents a priority of a [`Headline`]."]
    Priority => A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);

/// Represents an attribute for various other elements (currently only [`Headline`]).
///
/// Inlinetasks, items, planning, clocks, node properties and table rows can't have attributes.
///
/// The affilicated keywords are placed just above the item and follow one of the following
/// patterns:
///
/// * `#+KEY: VALUE`
/// * `#+KEY[OPTIONAL]: VALUE`
/// * `#+ATTR_BACKEND: VALUE`
///
/// `KEY` is either `CAPTION`, `HEADER`, `NAME`, `PLOT` or `RESULTS`.
///
/// `BACKEND` is a string constituted of alpha-numeric characters, hyphens or underscores.
///
/// `OPTIONAL` and `VALUE` can contain any character but a new line.
/// Only `CAPTION` and `RESULTS` keywords can have an optional value.
///
/// An affiliated keyword can appear more than once if `KEY` is either `CAPTION` or `HEADER`
/// or if its pattern is `#+ATTR_BACKEND: VALUE`.
///
/// `CAPTION`, `AUTHOR`, `DATE` and `TITLE` keywords can contain objects in their value
/// and their optional value, if applicable. (Objects are not implemented yet)
#[derive(Debug, PartialEq, Eq)]
pub struct AffiliatedKeyword {
    kind: AffiliatedKeywordKind,
    value: AffiliatedKeywordValue
}

impl AffiliatedKeyword {
    pub fn new(kind: AffiliatedKeywordKind, value: AffiliatedKeywordValue) -> Self {
        AffiliatedKeyword {
            kind, value
        }
    }
}

/// This represents the kind of a [`AffiliatedKeyword`].
#[derive(Debug, PartialEq, Eq)]
pub enum AffiliatedKeywordKind {
    /// The caption kind can have a optional value (See: [`AffiliatedKeyword`]).
    Caption(Option<AffiliatedKeywordValue>),
    Header,
    Name,
    Plot,
    /// The results kind can have an optional value (See: [`AffiliatedKeyword`]).
    Results(Option<AffiliatedKeywordValue>),
    /// The attr kind has a backend that is an arbitrary string.
    Attr(String),
}

/// This is a value of an [`AffiliatedKeyword`].
///
/// Currently this only wraps a string. This will contain objects in the future for
/// caption, author, date and title (optional) value.
#[derive(Debug, PartialEq, Eq)]
pub struct AffiliatedKeywordValue(String);

impl AffiliatedKeywordValue {
    pub fn new(value: impl Into<String>) -> Self {
        AffiliatedKeywordValue(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_to_z_is_parseable_to_priority() {
        use std::char;

        for i in 'A' as u32..('Z' as u32 + 1) {
            let prio = &char::from_u32(i).unwrap().to_string().parse::<Priority>();
            assert!(prio.is_ok());
        }
    }
}
