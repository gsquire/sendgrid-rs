use reqwest::{Client, ClientBuilder};

use crate::v3::Sender;

pub(crate) fn new_client(api_key: &str) -> Client {
    #[cfg(feature = "native-tls")]
    let c = ClientBuilder::new()
        .default_headers(Sender::get_headers(api_key).unwrap_or_default())
        .tls_backend_native()
        .build()
        .unwrap_or_default();

    #[cfg(not(feature = "native-tls"))]
    let c = ClientBuilder::new()
        .default_headers(Sender::get_headers(api_key).unwrap_or_default())
        .build()
        .unwrap_or_default();
    c
}

#[cfg(feature = "blocking")]
pub(crate) fn new_blocking_client(api_key: &str) -> reqwest::blocking::Client {
    #[cfg(feature = "native-tls")]
    let c = reqwest::blocking::ClientBuilder::new()
        .default_headers(Sender::get_headers(api_key).unwrap_or_default())
        .tls_backend_native()
        .build()
        .unwrap_or_default();

    #[cfg(not(feature = "native-tls"))]
    let c = reqwest::blocking::ClientBuilder::new()
        .default_headers(Sender::get_headers(api_key).unwrap_or_default())
        .build()
        .unwrap_or_default();

    c
}
