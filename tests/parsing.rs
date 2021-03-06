extern crate pest;
extern crate rust_orgmode;

use pest::Parser;
use rust_orgmode::parsing::{OrgModeParser, Rule};
use std::fs::{self, File};
use std::io::Read;

fn test_files() -> impl Iterator<Item = File> {
    fs::read_dir("tests/correct").unwrap().filter_map(|entry| {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return None,
        };
        let path = entry.path();
        if path.is_file() {
            if let Ok(file) = File::open(path) {
                Some(file)
            } else {
                None
            }
        } else {
            None
        }
    })
}

#[test]
fn parsing_succeeds() {
    test_files().for_each(|mut file| {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        OrgModeParser::parse(Rule::document, &contents).unwrap();
    })
}

#[test]
fn parsing_produces_document() {
    test_files().for_each(|mut file| {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        rust_orgmode::parsing::parse_document(&contents).unwrap();
    })
}
