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

impl<T> Fail for GenericError<T>
where
    T: 'static + Display + Debug + Send + Sync,
{
}

impl<T> From<T> for GenericError<T>
where
    T: 'static + Display + Debug + Send + Sync,
{
    fn from(e: T) -> Self {
        GenericError(e)
    }
}

//impl <T> From<T> for GenericError<T>
//where
//    T: Display + Debug + Send + Sync,
//{
//    fn from(e: T) -> Self {
//        GenericError(e)
//    }
//}

/// Translate parser result from IResult<I,O,_> to IResult<I,O,Error> with the [`Error`] type of the
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
///         to_failure!(err_test)
///       );
///
///     let a = &b"efghblah"[..];
///     println!("{:?}", parser(a));
///     //assert_eq!(parser(a), Err(Err::Error(Context::Code(a, ErrorKind::Custom(ErrorStr("custom error code: 42".to_string()))))));
/// # }
/// ```
#[macro_export]
macro_rules! to_failure (
    // The $i:expr is needed because nom injects the input if you use this macro inside e.g. named!
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        {
            use $crate::macros::GenericError;
            fix_error!($i, Error, fix_error!(GenericError<_>, $submac!( $($args)* )))
        }
    );
    ($i:expr, $e:expr) => (
        to_failure!($i, call!($e))
    );
);

/// Copied from nom source code and removed `u32` error type. This will compile with custom error
/// types.
///
/// ```
/// # #[macro_use] extern crate nom;
/// # #[macro_use] extern crate orgmode;
/// # use nom::IResult;
/// # use nom::Context;
/// # fn main() {
///     fn inner(i: &str) -> nom::IResult<&str, u32, bool> {
///         Err(nom::Err::Error(error_position!(i, nom::ErrorKind::Custom(false))))
///     }
///     named!(parser<&str, u32, bool>, complete!(inner));
/// # }
/// ```
#[macro_export]
macro_rules! complete (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        {
            use nom::lib::std::result::Result::*;
            use nom::{Err,ErrorKind};

            let i_ = $i.clone();
            match $submac!(i_, $($args)*) {
                Err(Err::Incomplete(_)) =>  {
                    Err(Err::Error(error_position!($i, ErrorKind::Complete)))
                },
                rest => rest
            }
        }
    );
    ($i:expr, $f:expr) => (
        complete!($i, call!($f));
    );
);
