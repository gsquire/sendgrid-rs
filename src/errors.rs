use failure::Fail;
use reqwest::{self, header::InvalidHeaderValue};
use std::io;

/// Represents any of the ways that using this library can fail.
#[derive(Fail, Debug)]
pub enum SendgridError {
    /// The failure was due to some IO error, for example an interrupted network connection.
    #[fail(display = "IO Error: {}", _0)]
    Io(#[cause] io::Error),

    /// The failure was due to invalid JSON being received.
    #[fail(display = "JSON Error: {}", _0)]
    JSONDecode(#[cause] serde_json::Error),

    /// The failure was due to the network client not working properly.
    #[fail(display = "HTTP Error: {}", _0)]
    ReqwestError(#[cause] reqwest::Error),

    /// The failure was due to the authorization headers not working as expected.
    #[fail(display = "Invalid Header Error: {}", _0)]
    InvalidHeader(#[cause] InvalidHeaderValue),

    /// The failure was due to a file containing invalid UTF-8.
    #[fail(display = "could not UTF-8 decode this filename")]
    InvalidFilename,

    /// SendGrid returned an unsuccessful HTTP status code.
    #[fail(display = "Request failed with StatusCode: {}", _0)]
    RequestNotSuccessful(reqwest::StatusCode, String),
}

impl From<reqwest::Error> for SendgridError {
    fn from(error: reqwest::Error) -> Self {
        SendgridError::ReqwestError(error)
    }
}

impl From<InvalidHeaderValue> for SendgridError {
    fn from(error: InvalidHeaderValue) -> Self {
        SendgridError::InvalidHeader(error)
    }
}

impl From<io::Error> for SendgridError {
    fn from(error: io::Error) -> Self {
        SendgridError::Io(error)
    }
}

impl From<serde_json::Error> for SendgridError {
    fn from(error: serde_json::Error) -> Self {
        SendgridError::JSONDecode(error)
    }
}

/// A type alias used throughout the library for concise error notation.
pub type SendgridResult<T> = Result<T, SendgridError>;
