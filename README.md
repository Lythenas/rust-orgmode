# Orgmode

[![Build Status](https://travis-ci.org/Lythenas/rust-orgmode.svg?branch=master)](https://travis-ci.org/Lythenas/rust-orgmode)

A rust library for parsing [org files](https://orgmode.org/).

**This library is currently not usable as not all necessary functions are implemented.**

I'm currently working to replace the existing types and how they are parsed. So probably
everything will be change.

## Planned features

- Parsing
- *Graceful* parsing (errors don't abort the parsing)
- Analyzing (warning reporting)
- Error/Warning correction

## Maybe future features

- Creating agenda
- Exporting

## References used for this library

- Generally https://orgmode.org
    - https://orgmode.org/worg/org-glossary.html
    - https://orgmode.org/org.html
    - https://orgmode.org/worg/dev/org-syntax.html
- Various examples
    - https://github.com/fniessen/refcard-org-mode
    - http://ehneilsen.net/notebook/orgExamples/org-examples.html
    - https://raw.githubusercontent.com/novoid/org-mode-workshop/master/featureshow/org-mode-teaser.org
    - my own org files
