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

pub type SendgridResult<T> = Result<T, Error>;
