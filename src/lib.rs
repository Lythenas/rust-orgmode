//! This is a library for working with [org files](https://orgmode.org/).
//!
//! Org files are on the surface like markdown files with different syntax.
//! However emacs org mode supports a lot more features than simple markdown.
//! In addition to simply being a markup and outlining language
//! > Org mode is for keeping notes, maintaining TODO lists, planning projects,
//! > and authoring documents with a fast and effective plain-text system.
//! >
//! > -- [org mode](https://orgmode.org/)
//!
//! This library is aimed to support most org mode features. But org mode is very
//! comprehensive.
//!
//! Currently only parsing of the major outline and timestamp is supported.
//!
//! # Todo
//!
//! - impl Object for every object
//! - impl Element for every element and greater element
//! - impl GreaterElement for every greater element
#![feature(plugin)]
#![feature(pattern)]
#![plugin(phf_macros)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
#[macro_use]
extern crate proptest;

//#[macro_use]
//extern crate failure;
//#[macro_use]
//extern crate failure_derive;
//#[macro_use]
//extern crate lazy_static;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod entities;
pub mod parsing;
pub mod types;
#[macro_use]
pub mod macros;

mod private {
    pub trait Sealed {}

    macro_rules! impl_sealed {
        ($ty:ty) => {
            impl crate::private::Sealed for $ty {}
        };
    }

    impl_sealed!(crate::types::objects::Entity);
    impl_sealed!(crate::types::objects::ExportSnippet);
    impl_sealed!(crate::types::objects::FootnoteReference);
    impl_sealed!(crate::types::objects::InlineBabelCall);
    impl_sealed!(crate::types::objects::InlineSrcBlock);
    impl_sealed!(crate::types::objects::LatexFragment);
    impl_sealed!(crate::types::objects::LineBreak);
    impl_sealed!(crate::types::objects::Link);
    impl_sealed!(crate::types::objects::Macro);
    impl_sealed!(crate::types::objects::RadioTarget);
    impl_sealed!(crate::types::objects::StatisticsCookie);
    impl_sealed!(crate::types::objects::Subscript);
    impl_sealed!(crate::types::objects::Superscript);
    impl_sealed!(crate::types::objects::TableCell);
    impl_sealed!(crate::types::objects::Target);
    impl_sealed!(crate::types::objects::TextMarkup);
    impl_sealed!(crate::types::objects::Timestamp);

    impl_sealed!(crate::types::elements::BabelCall);
    impl_sealed!(crate::types::elements::BlockFlags);
    impl_sealed!(crate::types::elements::Clock);
    impl_sealed!(crate::types::elements::Comment);
    impl_sealed!(crate::types::elements::CommentBlock);
    impl_sealed!(crate::types::elements::DiarySexp);
    impl_sealed!(crate::types::elements::ExampleBlock);
    impl_sealed!(crate::types::elements::ExportBlock);
    impl_sealed!(crate::types::elements::FixedWidth);
    impl_sealed!(crate::types::elements::HorizontalRule);
    impl_sealed!(crate::types::elements::Keyword);
    impl_sealed!(crate::types::elements::LatexEnvironment);
    impl_sealed!(crate::types::elements::NodeProperty);
    impl_sealed!(crate::types::elements::Paragraph);
    impl_sealed!(crate::types::elements::Planning);
    impl_sealed!(crate::types::elements::SrcBlock);

    impl_sealed!(crate::types::greater_elements::CenterBlock);
    impl_sealed!(crate::types::greater_elements::Drawer);
    impl_sealed!(crate::types::greater_elements::DynamicBlock);
    impl_sealed!(crate::types::greater_elements::FootnoteDefinition);
    impl_sealed!(crate::types::greater_elements::Headline);
    impl_sealed!(crate::types::greater_elements::Inlinetask);
    impl_sealed!(crate::types::greater_elements::Item);
    impl_sealed!(crate::types::greater_elements::PlainList);
    impl_sealed!(crate::types::greater_elements::PropertyDrawer);
    impl_sealed!(crate::types::greater_elements::QuoteBlock);
    impl_sealed!(crate::types::greater_elements::Section);
    impl_sealed!(crate::types::greater_elements::SpecialBlock);
    impl_sealed!(crate::types::greater_elements::Table);
    impl_sealed!(crate::types::greater_elements::TableRow);
    impl_sealed!(crate::types::greater_elements::VerseBlock);
}

#[cfg(test)]
mod tests {
    //use super::*;
}
