use std::io;

use reqwest;
use serde_json;

error_chain! {
    types {
        SendgridError, SendgridErrorKind, SendgridResultExt, SendgridResult;
    }

    foreign_links {
        Io(io::Error);
        JSONDecode(serde_json::Error);
        ReqwestError(reqwest::Error);
        HeaderError(reqwest::header::InvalidHeaderValue);
    }

    errors {
        InvalidFilename {
            description("invalid filename")
            display("could not UTF-8 decode this filename")
        }
    }
}
