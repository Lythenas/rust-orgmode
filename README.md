# Orgmode

[![Build Status](https://travis-ci.org/Lythenas/rust-orgmode.svg?branch=master)](https://travis-ci.org/Lythenas/rust-orgmode)

A rust library for parsing [org files](https://orgmode.org/).

**This library is currently not usable as not all necessary functions are implemented.**

I'm currently working to replace the existing types and how they are parsed. So probably
everything will be change.

## Planned features

- Parsing (Currently working on this)
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

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

