
/// Used as intermediate error type to convert between noms u32 and failures Error.
#[derive(Debug, PartialEq, Eq, Fail)]
#[fail(display = "Generic parse error: {}", _0)]
pub struct GenericError(u32);

impl From<u32> for GenericError {
    fn from(e: u32) -> GenericError {
        GenericError(e)
    }
}

/// Translate parser result from IResult<I,O,u32> to IResult<I,O,Error> with the [`Error`] type of the
/// failure crate.
///
/// ```
/// # #[macro_use] extern crate nom;
/// # extern crate failure;
/// # #[macro_use] extern crate failure_derive;
/// # #[macro_use] extern crate orgmode;
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
///         u32_to_failure!(err_test)
///       );
///
///     let a = &b"efghblah"[..];
///     println!("{:?}", parser(a));
///     //assert_eq!(parser(a), Err(Err::Error(Context::Code(a, ErrorKind::Custom(ErrorStr("custom error code: 42".to_string()))))));
/// # }
/// ```
#[macro_export]
macro_rules! u32_to_failure (
    // The $i:expr is needed because nom injects the input if you use this macro inside e.g. named!
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        {
            use $crate::macros::GenericError;
            fix_error!($i, Error, fix_error!(GenericError, $submac!( $($args)* )))
        }
    );
    ($i:expr, $e:expr) => (
        u32_to_failure!($i, call!($e))
    );
);
