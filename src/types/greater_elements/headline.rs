use super::*;

/// A headline.
///
/// # Semantics
///
/// The main element used to structure an org file. Also used as todo items/tasks. Can be
/// assigned a [`elements::Planning`] item to schedule an event.
///
/// If the first word of `TITLE` is `COMMENT` the headline will be considered as commented
/// (case is significant). If `TITLE` is `org-footnote-section` it will be considered as
/// the footnote section (case is significant).
///
/// If `TAGS` contains the `ARCHIVE` tag the headline will be considered archived (case is
/// significant).
///
/// # Syntax
///
/// Headlines are context-free.
///
/// ```text
/// STARS KEYWORD PRIORITY TITLE TAGS
/// ```
///
/// `STARS` is a string consisting of asterisks only. It has to start at the beginning of the
/// line. It contains at least one asterisk. This is the only required part of a headline. If
/// other parts of the headline follow there has to be at least a space after the stars.
///
/// `KEYWORD` is a todo keyword in all capital letters. If other parts of the headline follow
/// there has to be a single space after the keyword.
///
/// `PRIORITY` is a priority cookie of the form `[#A]` where `A` can be any letter. Capital
/// letters are recommended.
///
/// `TITLE` can contain any character but a newline. Title will be parsed as secondary string
/// and can contain the standard set of objects without line breaks.
///
/// `TAGS` is made of strings containing any alpha-numeric character, underscores, at signs,
/// hash signs and percent signs. Tags are separated and surrounded by `:`s. There can be an
/// arbitraty amount of whitespace (except newlines) between `TITLE` and `TAGS`. Tags are
/// usually right aligned at a specified column by the editor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Headline {
    pub(crate) affiliated_keywords: Option<Spanned<AffiliatedKeywords>>,
    pub level: u32,
    pub todo_keyword: Option<TodoKeyword>,
    pub priority: Option<char>, // TODO maybe make separate struct
    pub title: Option<SecondaryString<StandardSetNoLineBreak>>,
    pub tags: Vec<String>,
    pub planning: Option<elements::Planning>,
    pub property_drawer: Option<PropertyDrawer>,
    pub(crate) content: Option<Spanned<Vec<HeadlineContentSet>>>,
    // quotedp ?
    // hiddenp: bool,
    // pre_blank: u32 // TODO (maybe) blank lines before the content starts
}

impl Parent<Vec<HeadlineContentSet>> for Headline {
    fn content(&self) -> Option<&Spanned<Vec<HeadlineContentSet>>> {
        self.content.as_ref()
    }
}

impl Headline {
    pub fn is_footnote_section(&self) -> bool {
        self.title
            .as_ref()
            .map(|title| title == "org-footnote-section")
            .unwrap_or(false)
    }
    pub fn is_commented(&self) -> bool {
        self.title
            .as_ref()
            .map(|title| title.starts_with("COMMENT"))
            .unwrap_or(false)
    }
    pub fn is_archived(&self) -> bool {
        self.tags.contains(&"ARCHIVE".to_string())
    }

    pub fn push_content(&mut self, content: impl IntoIterator<Item=HeadlineContentSet>) {
        self.content
            .get_or_insert_with(|| Spanned::new(Vec::new()))
            .get_mut_value()
            .extend(content);
    }
}

/// List of elements that are content of a [`Headline`] or [`Inlinetask`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HeadlineContentSet {
    Section(greater_elements::Section),
    Headline(Box<greater_elements::Headline>),
}

/// A todo keyword of a [`Headline`] or [`Inlinetask`].
///
/// Todo keywords can be configured before parsing. The default is to parse `TODO` and `NEXT` as
/// **`Todo`** and `DONE` as **`Done`**. The actual keyword used is the string in the variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TodoKeyword {
    /// Usually parsed from `TODO` and `NEXT`.
    Todo(String),
    /// Usually parsed from `DONE`.
    Done(String),
}

/// An inline task.
///
/// # Semantics
///
/// Similar to a [`Headline`] but can have a defined end. Headlines end when the next starts or
/// the document ends.
///
/// # Syntax
///
/// Same syntax as [`Headline`] but starts with at least *org-inlinetask-min-level* astersisks.
/// This variable is currently not implemented. Inline tasks can also not be commented or
/// archived.
///
/// Inline tasks can be ended with a line of *org-inlinetask-min-level* asterisks followed by a
/// space and the string `END`. This should start at the beginning of a line but that is not
/// required.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inlinetask {
    content: Spanned<Vec<HeadlineContentSet>>,
    pub todo_keyword: Option<TodoKeyword>,
    pub priority: Option<char>, // TODO maybe make separate struct (maybe use old enum)
    pub title: Option<SecondaryString<StandardSetNoLineBreak>>,
    pub tags: Vec<String>,
    // hiddenp: bool,
    // pre_blank: u32 // blank lines before the content starts
}

impl Parent<Vec<HeadlineContentSet>> for Inlinetask {
    fn content(&self) -> Option<&Spanned<Vec<HeadlineContentSet>>> {
        Some(&self.content)
    }
}
