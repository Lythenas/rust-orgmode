# Orgmode

[![Build Status](https://travis-ci.org/Lythenas/rust-orgmode.svg?branch=master)](https://travis-ci.org/Lythenas/rust-orgmode)

A rust library for parsing [org files](https://orgmode.org/).

**This library is currently not usable as not all necessary functions are implemented.**

## Basic features

- [x] Parsing
    - [x] Timestamp
    - [x] Headlines
    - [x] Affiliated Keywords
- [ ] *Graceful* parsing (report errors but don't abort parsing)
- [ ] Analyzing
    - [ ] Inheritance
    - [ ] Searching API (e.g. for tags, properties, etc.)
    - [ ] Grouping and sorting (e.g. by tags, category, date)
    - [ ] Resolving attachments
- [ ] Error correction
- [ ] Exporting

## Planned (future) features

- Org-mode agenda
- Content syntax parsing (e.g. to extract tables, links or images)

## References used for this library

- Generally https://orgmode.org
    - https://orgmode.org/worg/org-glossary.html
    - https://orgmode.org/org.html
    - https://orgmode.org/worg/dev/org-syntax.html
- Various examples
    - https://github.com/fniessen/refcard-org-mode
    - http://ehneilsen.net/notebook/orgExamples/org-examples.html
    - https://raw.githubusercontent.com/novoid/org-mode-workshop/master/featureshow/org-mode-teaser.org
    - my own files
