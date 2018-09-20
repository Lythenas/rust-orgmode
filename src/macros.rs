use failure::Fail;
use std::fmt::{self, Debug, Display};

/// Used as intermediate error type for the to_failure macro.
#[derive(Debug, PartialEq, Eq)]
pub struct GenericError<T>(T);

impl<T> Display for GenericError<T>
where
    T: Display + Debug + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> Fail for GenericError<T> where T: 'static + Display + Debug + Send + Sync {}

impl<T> From<T> for GenericError<T>
where
    T: 'static + Display + Debug + Send + Sync,
{
    fn from(e: T) -> Self {
        GenericError(e)
    }
}

/// Translate parser result from IResult<I,O,_> to IResult<I,O,Error> with the [`Error`] type of the
/// failure crate.
///
/// ```
/// # #[macro_use] extern crate nom;
/// # extern crate failure;
/// # #[macro_use] extern crate failure_derive;
/// # #[macro_use] extern crate rust_orgmode;
/// # use nom::IResult;
/// # use nom::Context;
/// # use nom::Err;
/// # use nom::ErrorKind;
/// # use failure::Error;
/// # fn main() {
///     // will add a Custom(42) error to the error chain
///     named!(err_test, add_return_error!(ErrorKind::Custom(42u32), tag!("abcd")));
///
///     named!(parser<&[u8], &[u8], Error>,
///         to_failure!(err_test)
///       );
///
///     let a = &b"efghblah"[..];
///     println!("{:?}", parser(a));
///     //assert_eq!(parser(a), Err(Err::Error(Context::Code(a, ErrorKind::Custom(ErrorStr("custom error code: 42".to_string()))))));
/// # }
/// ```
///
/// [`Error`]: failure::Error
#[macro_export]
macro_rules! to_failure (
    // The $i:expr is needed because nom injects the input if you use this macro inside e.g. named!
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        {
            use $crate::macros::GenericError;
            use failure::Error;
            fix_error!($i, Error, fix_error!(GenericError<_>, $submac!( $($args)* )))
        }
    );
    ($i:expr, $e:expr) => (
        to_failure!($i, call!($e))
    );
);

/// `take_until_or_eof!(tag) => T -> IResult<T, T>`
/// consumes data until it finds the specified tag or everything if the
/// input does not contain the tag.
///
/// The remainder still contains the tag.
///
/// # Example
/// ```
/// # #[macro_use] extern crate nom;
/// # #[macro_use] extern crate rust_orgmode;
/// # fn main() {
///  named!(x, take_until_or_eof!("foo"));
///  let r = x(&b"abcd foo efgh"[..]);
///  assert_eq!(r, Ok((&b"foo efgh"[..], &b"abcd "[..])));
/// # }
/// ```
#[macro_export]
macro_rules! take_until_or_eof (
    ($i:expr, $substr:expr) => {{
        use nom::IResult;
        use nom::InputLength;
        use nom::FindSubstring;
        use nom::InputTake;
        let input = $i;

        let res: IResult<_,_> = match input.find_substring($substr) {
            None => {
                Ok($i.take_split(input.input_len()))
            },
            Some(index) => {
                Ok($i.take_split(index))
            },
        };
        res
    }};
);

#[cfg(test)]
mod tests {
    use nom::types::CompleteStr;

    #[test]
    fn test_take_until_or_eof() {
        named!(x<CompleteStr, CompleteStr>, take_until_or_eof!("\n"));
        let r = x(CompleteStr("abc def"));
        assert_eq!(r, Ok((CompleteStr(""), CompleteStr("abc def"))));
        let r = x(CompleteStr("abc\ndef"));
        assert_eq!(r, Ok((CompleteStr("\ndef"), CompleteStr("abc"))));
    }
}
