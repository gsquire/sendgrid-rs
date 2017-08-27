#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate data_encoding;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod errors;
pub mod mail;
pub mod sg_client;
pub mod v3;
