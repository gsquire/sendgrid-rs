use std::io;

use reqwest;
use serde_json;
use failure::Error;

#[derive(Fail, Debug)]
pub enum SendgridError {
    #[fail(display = "IO Error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "JSON Error: {}", _0)]
    JSONDecode(#[cause] serde_json::Error),
    #[fail(display = "HTTP Error: {}", _0)]
    ReqwestError(#[cause] reqwest::Error),
    #[fail(display = "could not UTF-8 decode this filename")]
    InvalidFilename,
}

impl From<reqwest::Error> for SendgridError {
    fn from(error: reqwest::Error) -> Self {
        SendgridError::ReqwestError(error)
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

pub type SendgridResult<T> = Result<T, SendgridError>;
