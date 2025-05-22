use reqwest::{Client, ClientBuilder};

use crate::v3::Sender;

pub(crate) fn new_client(api_key: &str) -> Client {
    ClientBuilder::new()
        .default_headers(Sender::get_headers(api_key).unwrap_or_default())
        .build()
        .unwrap_or_default()
}

#[cfg(feature = "blocking")]
pub(crate) fn new_blocking_client(api_key: &str) -> reqwest::blocking::Client {
    reqwest::blocking::ClientBuilder::new()
        .default_headers(Sender::get_headers(api_key).unwrap_or_default())
        .build()
        .unwrap_or_default()
}
