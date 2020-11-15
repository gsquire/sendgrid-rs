use std::{
    fmt::{self, Display},
    io,
};

use reqwest::{self, header::InvalidHeaderValue, StatusCode};
use thiserror::Error as ThisError;

/// Wrapper type which contains a failed request's status code and body.
#[derive(Debug)]
pub struct RequestNotSuccessful {
    /// Status code returned by the HTTP call to the SendGrid API.
    pub status: StatusCode,
    /// Body returned by the HTTP call to the SendGrid API.
    pub body: String,
}

impl RequestNotSuccessful {
    /// Create a new unsuccessful request error.
    pub fn new(status: StatusCode, body: String) -> Self {
        Self { status, body }
    }
}

impl std::error::Error for RequestNotSuccessful {}

impl Display for RequestNotSuccessful {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusCode: {}, Body: {}", self.status, self.body)
    }
}

/// Represents any of the ways that using this library can fail.
#[derive(ThisError, Debug)]
pub enum SendgridError {
    /// The failure was due to some IO error, for example an interrupted network connection.
    #[error("IO Error: `{0}`")]
    Io(#[from] io::Error),

    /// The failure was due to invalid JSON being received.
    #[error("JSON Error: `{0}`")]
    JSONDecode(#[from] serde_json::Error),

    /// The failure was due to the network client not working properly.
    #[error("HTTP Error: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    /// The failure was due to the authorization headers not working as expected.
    #[error("Invalid Header Error: `{0}`")]
    InvalidHeader(#[from] InvalidHeaderValue),

    /// The failure was due to a file containing invalid UTF-8.
    #[error("could not UTF-8 decode this filename")]
    InvalidFilename,

    /// SendGrid returned an unsuccessful HTTP status code.
    #[error("Request failed: `{0}`")]
    RequestNotSuccessful(#[from] RequestNotSuccessful),
}

/// A type alias used throughout the library for concise error notation.
pub type SendgridResult<T> = Result<T, SendgridError>;
