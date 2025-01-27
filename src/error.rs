// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::fmt;
use std::io;

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error(ErrorKind);

// Hiding error variants from a library's public error type to prevent
// dependency updates from becoming breaking changes.
// We can add `is_*` methods that indicate the kind of error if needed, but
// don't expose dependencies' types directly in the public API.
#[derive(Debug)]
pub(crate) enum ErrorKind {
    Io(io::Error),

    Json(serde_json::Error),

    Other(Box<str>),
    WithContext(Box<str>, Option<Box<dyn std::error::Error + Send + Sync + 'static>>),
}

impl Error {
    pub(crate) fn new(e: impl Into<ErrorKind>) -> Self {
        Self(e.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorKind::Io(e) => fmt::Display::fmt(e, f),
            ErrorKind::Json(e) => fmt::Display::fmt(e, f),
            ErrorKind::Other(e) | ErrorKind::WithContext(e, ..) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0 {
            ErrorKind::Io(e) => e.source(),
            ErrorKind::Json(e) => e.source(),
            ErrorKind::Other(_) => None,
            ErrorKind::WithContext(_, e) => Some(&**e.as_ref()?),
        }
    }
}

impl From<io::Error> for ErrorKind {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<String> for ErrorKind {
    fn from(e: String) -> Self {
        Self::Other(e.into_boxed_str())
    }
}
impl From<serde_json::Error> for ErrorKind {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

// Note: Do not implement From<ThirdPartyErrorType> to prevent dependency
// updates from becoming breaking changes.
// Implementing `From<StdErrorType>` should also be avoided whenever possible,
// as it would be a breaking change to remove the implementation if the
// conversion is no longer needed due to changes in the internal implementation.

// Inspired by anyhow::Context.
pub(crate) trait Context<T, E> {
    // fn context<C>(self, context: C) -> Result<T, Error>
    // where
    //     C: fmt::Display;
    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: fmt::Display,
        F: FnOnce() -> C;
}
impl<T, E> Context<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    // fn context<C>(self, context: C) -> Result<T, Error>
    // where
    //     C: fmt::Display,
    // {
    //     match self {
    //         Ok(ok) => Ok(ok),
    //         Err(e) => Err(Error(ErrorKind::WithContext(
    //             context.to_string().into_boxed_str(),
    //             Some(Box::new(e)),
    //         ))),
    //     }
    // }
    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: fmt::Display,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(Error(ErrorKind::WithContext(
                context().to_string().into_boxed_str(),
                Some(Box::new(e)),
            ))),
        }
    }
}
impl<T> Context<T, core::convert::Infallible> for Option<T> {
    // fn context<C>(self, context: C) -> Result<T, Error>
    // where
    //     C: fmt::Display,
    // {
    //     match self {
    //         Some(ok) => Ok(ok),
    //         None => Err(Error(ErrorKind::WithContext(context.to_string().into_boxed_str(), None))),
    //     }
    // }
    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: fmt::Display,
        F: FnOnce() -> C,
    {
        match self {
            Some(ok) => Ok(ok),
            None => {
                Err(Error(ErrorKind::WithContext(context().to_string().into_boxed_str(), None)))
            }
        }
    }
}
