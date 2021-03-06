//! Helper macros. Note that these are relatively new and may change in a later version.
//!
//! The idea is to use `itry` for internal server operations which can't be recovered from, and
//! `iexpect` for validating user input. Note that this kind of usage is completely non-normative.
//! Feedback about actual usability and usage is apprechiated.

/// Like try!(), but wrapping the error value in `IronError`. To be used in request handlers.
///
/// ```ignore
/// let f = itry!(fs::File::create("foo.txt"), status::BadRequest);
/// let f = itry!(fs::File::create("foo.txt"));  // Default modifier is status::InternalServerError
/// ```
///
#[macro_export]
macro_rules! itry {
    ($result:expr) => (itry!($result, $crate::status::InternalServerError));

    ($result:expr, $modifier:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => return ::std::result::Result::Err(
            $crate::IronError::new(err, $modifier))
    })
}

/// Unwrap the given Option or return a Ok(Response::new()) with the given modifier. The default
/// modifier is status::BadRequest.
#[macro_export]
macro_rules! iexpect {
    ($option:expr) => (iexpect!($option, $crate::status::BadRequest));
    ($option:expr, $modifier:expr) => (match $option {
        ::std::option::Option::Some(x) => x,
        ::std::option::Option::None => return ::std::result::Result::Ok(
            $crate::response::Response::with($modifier))
    })
}
