#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate data_encoding;

pub mod sg_client;
pub mod mail;
pub mod v3;
