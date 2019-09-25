#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure_derive;

extern crate data_encoding;
extern crate failure;
#[cfg(feature = "async")]
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod errors;
mod mail;
mod sg_client;
pub mod v3;

pub use crate::mail::{Destination, Mail};
pub use crate::sg_client::SGClient;
