use failure::Error;
use nom::types::CompleteStr;

use {AffiliatedKeyword, AffiliatedKeywordKind, AffiliatedKeywordValue};

//trace_macros!(true);

named!(#[doc="
Parses an affiliated keyword kind.

Has one of the formats:

* `KEY`
* `KEY[OPTIONAL]`
* `ATTR_BACKEND`

`KEY` is either `CAPTION`, `HEADER`, `NAME`, `PLOT` or `RESULTS`.

`OPTIONAL` is only allowed when `KEY` is `CAPTION` or `RESULTS`.

`BACKEND` is a alpha-numeric string with hyphens and underscores.
"],
kind<CompleteStr, AffiliatedKeywordKind, Error>,
    to_failure!(alt!(
        do_parse!(
            to_failure!(tag!("CAPTION")) >>
            optional: opt!(delimited!(
                to_failure!(tag!("[")),
                optional,
                to_failure!(tag!("]"))
            )) >>
            (AffiliatedKeywordKind::Caption(optional))
        ) |
        do_parse!(
            to_failure!(tag!("RESULTS")) >>
            optional: opt!(delimited!(
                to_failure!(tag!("[")),
                optional,
                to_failure!(tag!("]"))
            )) >>
            (AffiliatedKeywordKind::Results(optional))
        ) |
        to_failure!(do_parse!(
            tag!("HEADER") >>
            (AffiliatedKeywordKind::Header)
        )) |
        to_failure!(do_parse!(
            tag!("NAME") >>
            (AffiliatedKeywordKind::Name)
        )) |
        to_failure!(do_parse!(
            tag!("PLOT") >>
            (AffiliatedKeywordKind::Plot)
        )) |
        to_failure!(do_parse!(
            tag!("ATTR_") >>
            backend: take_until_or_eof!(":") >>
            (AffiliatedKeywordKind::Attr(backend.to_string()))
        ))
    ))
);

trace_macros!(false);

named!(#[doc="
Parses an affiliate keyword value.

Value can contain any char except newline.
"],
value<CompleteStr, AffiliatedKeywordValue, Error>,
    to_failure!(do_parse!(
        value: take_until_or_eof!("\n") >>
        (AffiliatedKeywordValue::new(*value))
    ))
);

named!(#[doc="
Parses an affiliate keyword optional value.

Value can contain any char except newline.
"],
optional<CompleteStr, AffiliatedKeywordValue, Error>,
    to_failure!(do_parse!(
        value: take_until_or_eof!("]") >>
        (AffiliatedKeywordValue::new(*value))
    ))
);

named!(#[doc="
Parses an affiliated keyword.

Has one of the formats:

* `#+KEY: VALUE`
* `#+KEY[OPTIONAL]: VALUE`
* `#+ATTR_BACKEND: VALUE`
"],
pub single_affiliated_keyword<CompleteStr, AffiliatedKeyword, Error>,
    to_failure!(do_parse!(
        to_failure!(tag!("#+")) >>
        kind: kind >>
        to_failure!(tag!(": ")) >>
        value: value >>
        (
            AffiliatedKeyword::new(kind, value)
        )
    ))
);

named!(#[doc="
Parses multiple affiliated keywords.

Does not check if the keywords are repeated. Normally only `CAPTION`,
`HEADER` and `ATTR_BACKEND` keywords can be repeated.

See: [`single_affiliated_keyword`]
"],
pub affiliated_keywords<CompleteStr, Vec<AffiliatedKeyword>, Error>,
    separated_list!(
        to_failure!(tag!("\n")),
        single_affiliated_keyword
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(
            value(CompleteStr("this is a value")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeywordValue::new("this is a value")
            ))
        );
        assert_eq!(
            value(CompleteStr("this is a value\nrest")).ok(),
            Some((
                CompleteStr("\nrest"),
                AffiliatedKeywordValue::new("this is a value")
            ))
        );
    }

    #[test]
    fn test_kind() {
        assert_eq!(
            kind(CompleteStr("CAPTION")).ok(),
            Some((CompleteStr(""), AffiliatedKeywordKind::Caption(None)))
        );
        assert_eq!(
            kind(CompleteStr("CAPTION[something]")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeywordKind::Caption(Some(AffiliatedKeywordValue::new("something")))
            ))
        );
        assert_eq!(
            kind(CompleteStr("HEADER")).ok(),
            Some((CompleteStr(""), AffiliatedKeywordKind::Header))
        );
        assert_eq!(
            kind(CompleteStr("NAME")).ok(),
            Some((CompleteStr(""), AffiliatedKeywordKind::Name))
        );
        assert_eq!(
            kind(CompleteStr("PLOT")).ok(),
            Some((CompleteStr(""), AffiliatedKeywordKind::Plot))
        );
        assert_eq!(
            kind(CompleteStr("RESULTS")).ok(),
            Some((CompleteStr(""), AffiliatedKeywordKind::Results(None)))
        );
        assert_eq!(
            kind(CompleteStr("RESULTS[something]")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeywordKind::Results(Some(AffiliatedKeywordValue::new("something")))
            ))
        );
        assert_eq!(
            kind(CompleteStr("ATTR_backend")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeywordKind::Attr("backend".to_string())
            ))
        );
    }

    #[test]
    fn test_single_affiliated_keyword() {
        // `#+KEY: VALUE`
        // `#+KEY[OPTIONAL]: VALUE`
        // `#+ATTR_BACKEND: VALUE`
        assert_eq!(
            single_affiliated_keyword(CompleteStr("#+HEADER: some header")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeyword::new(
                    AffiliatedKeywordKind::Header,
                    AffiliatedKeywordValue::new("some header")
                )
            ))
        );
        assert_eq!(
            single_affiliated_keyword(CompleteStr("#+HEADER: some header\nmore")).ok(),
            Some((
                CompleteStr("\nmore"),
                AffiliatedKeyword::new(
                    AffiliatedKeywordKind::Header,
                    AffiliatedKeywordValue::new("some header")
                )
            ))
        );
        assert_eq!(
            single_affiliated_keyword(CompleteStr("#+CAPTION: some caption")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeyword::new(
                    AffiliatedKeywordKind::Caption(None),
                    AffiliatedKeywordValue::new("some caption")
                )
            ))
        );
        assert_eq!(
            single_affiliated_keyword(CompleteStr("#+CAPTION[opt]: some caption")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeyword::new(
                    AffiliatedKeywordKind::Caption(Some(AffiliatedKeywordValue::new("opt"))),
                    AffiliatedKeywordValue::new("some caption")
                )
            ))
        );
        assert_eq!(
            single_affiliated_keyword(CompleteStr("#+ATTR_backend: some value")).ok(),
            Some((
                CompleteStr(""),
                AffiliatedKeyword::new(
                    AffiliatedKeywordKind::Attr("backend".to_string()),
                    AffiliatedKeywordValue::new("some value")
                )
            ))
        );
    }
}
