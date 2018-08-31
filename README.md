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
- [ ] Error correction
- [ ] Exporting

## Planned (future) features

- Org-mode agenda
- Content syntax parsing (e.g. to extract tables, links or images)
