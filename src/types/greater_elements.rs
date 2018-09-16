//! Contains all greater elements.

use super::*;
use rust_orgmode_derive::add_fields_for;

/// A center block.
///
/// # Semantics
///
/// Centers text. Also the content can contain markup.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_CENTER
/// CONTENTS
/// #+END_CENTER
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(Element, HasAffiliatedKeywords)]
pub struct CenterBlock {
    content_data: ContentData, // TODO only allow the standard set of elements
}

/// A drawer to hide content.
///
/// # Semantics
///
/// Used to hide content in the editor and when exporting. Drawers can usually be opened and
/// closed in the editor.
///
/// # Syntax
///
/// ```text
/// :NAME:
/// CONTENTS
/// :END:
/// ```
///
/// `NAME` can contain any word-constituent characters, hyphens and underscores.
///
/// `CONTENTS` can contain any element except a [`Headline`] and another drawer.
///
/// Drawers can be indented.
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct Drawer {
    name: String,
    // hiddenp: bool,
}

/// A dynamic block.
///
/// # Semantics
///
/// The content of dynamic blocks can be updated automatically by calling the a function with
/// the given name and parameters. It that function needs the previous content of the block an
/// extra parameter `:content` has to be added.
///
/// # Syntax
///
/// ```text
/// #+BEGIN: NAME PARAMETERS
/// CONTENTS
/// #+END:
/// ```
///
/// Note the `:` after `BEGIN` and `END`. It can be omitted after `END` without generating an
/// error.
///
/// `NAME` can contain anything except whitespace.
///
/// `PARAMETERS` can contain any character and can be omitted. They are usually of the format
/// `:name value` or `:name`.
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct DynamicBlock {
    /// The name of the function that can update this block.
    name: String,
    /// The parameters to pass to the function updating this block.
    ///
    /// Usually of the format `:name value`, separated by a space. Value can also be omitted.
    ///
    /// If the function needs the current content of the block add a parameter `:content`.
    parameters: String, // TODO maybe parse this as a list
    // hiddenp: bool
}

/// A footnote definition.
///
/// # Semantics
///
/// Defines a footnote that can be references with a [`objects::FootnoteReference`].
///
/// # Syntax
///
/// ```text
/// [LABEL] CONTENTS
/// ```
///
/// `LABEL` is either a number or follows the pattern `fn:WORD` where `WORD` can contain any
/// word-constituent character, hyphens and underscores.
///
/// `CONTENTS` can contain any element except another footnote definition and a [`Headline`].
/// It ends at the next footnote definition, headline, with two consecutive empty lines or the
/// end of the buffer.
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct FootnoteDefinition {
    label: String,
    // pre_blank: u32 // blank lines after `[LABEL]`
}

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
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct Headline {
    level: u32,
    todo_keyword: Option<TodoKeyword>,
    priority: Option<char>, // TODO maybe make separate struct
    title: Option<SecondaryString>,
    tags: Vec<String>,
    // TODO add reference to the associated planning element
    archived: bool,
    commented: bool,
    footnote_section: bool,
    // TODO add reference to the property drawer

    // quotedp ?
    // hiddenp: bool,
    // pre_blank: u32 // blank lines before the content starts
}

/// A todo keyword of a [`Headline`] or [`Inlinetask`].
///
/// Todo keywords can be configured before parsing. The default is to parse `TODO` and `NEXT` as
/// **`Todo`** and `DONE` as **`Done`**. The actual keyword used is the string in the variant.
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
#[derive(GreaterElement, getters)]
#[add_fields_for(GreaterElement)]
pub struct Inlinetask {
    todo_keyword: Option<TodoKeyword>,
    priority: Option<char>, // TODO maybe make separate struct
    title: Option<SecondaryString>,
    tags: Vec<String>,

    // hiddenp: bool,
    // pre_blank: u32 // blank lines before the content starts
}

/// An item in a list.
///
/// # Semantics
///
/// This is an item in a list.
///
/// # Syntax
///
/// ```text
/// BULLET [@COUNTER] [CHECKBOX] TAG
/// ```
///
/// `BULLET` is either an asterisk, a hyphen, a plus sign (for unordered lists) or follows the
/// pattern `COUNTER.` or `COUNTER)` (for ordered lists). This is the only required part of
/// item. `BULLET` is always followed by a whitespace or end of line.
///
/// `COUNTER` is a number or a single letter.
///
/// `CHECKBOX` is either a single whitespace, a `X` or a hyphen.
///
/// `TAG` follows the pattern `TAG-TEXT ::` where `TAG-TEXT` can contain any character except a
/// newline.
///
/// An item ends before the next item, the first line that is less or equally indented that its
/// starting line or two consecutive empty lines. Indentation of lines within other greater
/// elements including inlinetask boundaries are ignored.
#[derive(GreaterElement, getters)]
#[add_fields_for(GreaterElement)]
pub struct Item {
    // TODO move all of this to an enum to make more typesafe
    bullet: String, // TODO make struct
    checkbox: Option<Checkbox>,
    counter: Option<String>, // TODO make struct
    tag: Option<String>,
    // structure ?
    // hiddenp: bool
}

// TODO find better names for the variants
/// Checkbox of an [`Item`] in a list.
pub enum Checkbox {
    /// A space. (Empty checkbox)
    Off,
    /// A `X`. (Checked checkbox)
    X,
    /// A `-`. (Half checked checkbox? or disabled checkbox?)
    Trans,
}

/// A plain list.
///
/// # Semantics
///
/// A complete list of [`Item`]s.
///
/// # Syntax
///
/// This is a set of consecutive items of the same indentation. It can only directly contain
/// items.
///
/// If the dirst item has a `COUNTER` in its `BULLET` the plain list is be an *ordered plain
/// list*. If it contains a tag it is be a *descriptive list*. Otherwise it is be an
/// *unordered list*.
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct PlainList {
    // TODO content is only items
    // structure ?
}

impl PlainList {
    pub fn kind(&self) -> ListKind {
        // find first item if any
        // get kind of item and return it
        unimplemented!()
    }
}

/// The list kind of a [`PlainList`].
pub enum ListKind {
    Unordered,
    UnorderedDescriptive,
    Ordered,
    OrderedDescriptive,
}

/// A property drawer.
///
/// # Semantics
///
/// A drawer associated with a [`Headline`]. It contains attributes of a headline.
///
/// # Syntax
///
/// Property drawers are context-free.
///
/// ```text
/// :PROPERTIES:
/// CONTENTS
/// :END:
/// ```
///
/// `CONTENTS` consists of zero or more [`elements::NodeProperty`].
#[derive(GreaterElement, getters)]
#[add_fields_for(GreaterElement)]
pub struct PropertyDrawer {
    // TODO make this so only node properties are allowed in the content
    // hiddenp: bool
}

/// A quote.
///
/// # Semantics
///
/// Used for quotes. When exporting this block will be indented on the left and right margin.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_QUOTE
/// CONTENTS
/// #+END_QUOTE
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// TODO not sure if this is actually a greater element
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct QuoteBlock {
    // hiddenp: bool
}

/// A section.
///
/// # Semantics
///
/// This is a container for the content after a [`Headline`] or at the beginning of an org file
/// before the first headline.
///
/// # Syntax
///
/// Sections are context-free.
///
/// A section contains directly any (greater) element. Only a [`Headline`] can contain a
/// section. Also content before the first headline in a document belongs to a section.
///
/// A section ends at the beginning of the next headline or the end of the file.
#[derive(GreaterElement, getters)]
#[add_fields_for(GreaterElement)]
pub struct Section {
}

/// A special block.
///
/// # Semantics
///
/// Any block with name that is not recognized as another block is a special block.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_NAME
/// CONTENTS
/// #+END_NAME
/// ```
///
/// `NAME` can contain any non-whitespace character.
///
/// `CONTENTS` can contain anything except a line `#+END_CENTER` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will not be parsed.
///
/// TODO not sure if this is actually a greater element
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct SpecialBlock {
    kind: String,
    // hiddenp: bool
}

/// A table.
///
/// # Semantics
///
/// There are two types of tables:
///
/// - **org tables** can only contain [`TableRow`]s.
/// - **table.el tables** don't have parsed content.
///
/// # Syntax
///
/// Tables start with a line starting with a vertical bar or the string `+-` followed by plus
/// or binus signs only. Tables can be indented. The second line determines what type of table
/// this is.
///
/// # Org tables
///
/// Org tables start with a line starting with `|` and end at the first line not starting
/// with a vertical bar. They can be immediately followed by `#+TBLFM: FORMULAS` lines where
/// `FORMULAS` can contain any character.
///
/// ## Example
///
/// ```text
/// | col1 | col2 | col3 |
/// |------+------+------|
/// |  200 |  300 |  500 |
/// #+TBLFM: $3=$1+$2
/// ```
///
/// # Table.el tables
/// Table.el tables lines start with either a `|` or `+`. And end at the first line not
/// starting with either a vertical bar or a plus sign.
///
/// ## Example
///
/// ```text
/// +------+------+------+
/// | col1 | col2 | col3 |
/// +------+------+------+
/// |  200 |  300 |  500 |
/// +------+------+------+
/// ```
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(GreaterElement, HasAffiliatedKeywords)]
pub struct Table {
    // TODO maybe make this a bit more type safe
    kind: TableKind,
    /// Empty if this is a org table.
    formulas: Vec<String>,
    /// `None` if this is a org table.
    value: Option<String>,
}

/// The kind of a [`Table`].
pub enum TableKind {
    Org,
    TableEl,
}

/// A row in a [`Table`][`Table`].
///
/// # Semantics
///
/// A row contains cell which can contain content.
///
/// # Syntax
///
/// There are two kinds of table rows:
///
/// - normal: vertical bar and any number of [`TableCell`][`objects::TableCell`]s
///   ```text
///   | cell 1 | cell 2 | ... |
///   ```
/// - a rule: vertical bar followed by hyphens followed by a vertical bar
///   ```text
///   |--------|
///   ```
#[derive(GreaterElement, getters)]
#[add_fields_for(SharedBehavior)]
pub struct TableRow {
    kind: TableRowKind,
    content_data: ContentData, // TODO only allow TableCells
}

/// The kind of a [`TableRow`].
pub enum TableRowKind {
    Normal,
    Rule,
}

/// A verse block.
///
/// # Semantics
///
/// Simmilar to an [`elements::ExampleBlock`] but content is interpreted as objects. Verse blocks
/// preserve indentation.
///
/// # Syntax
///
/// ```text
/// #+BEGIN_VERSE
/// CONTENTS
/// #+END_VERSE
/// ```
///
/// `CONTENTS` can contain anything except a line `#+END_VERSE` on its own. Lines beginning
/// with stars must be quoted by comma. `CONTENTS` will be parsed as objects.
#[derive(GreaterElement, HasAffiliatedKeywords, getters)]
#[add_fields_for(SharedBehavior, HasAffiliatedKeywords)]
pub struct VerseBlock {
    content_data: ContentData, // TODO only allow the standard set of objects
}

