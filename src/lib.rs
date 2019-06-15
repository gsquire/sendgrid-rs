#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure_derive;

extern crate data_encoding;
extern crate failure;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod errors;
mod mail;
pub mod v3;

pub use mail::{Destination, Mail};

#[cfg(not(feature = "async"))]
mod sg_client;
#[cfg(not(feature = "async"))]
pub use sg_client::SGClient;

#[cfg(feature = "async")]
mod sg_async_client;
#[cfg(feature = "async")]
pub use sg_async_client::SGClient;

// Note: Ideally we would support both a blocking client and
//       an async client without a feature flag, but the originally
//       discussed solution requires Generic Associated Types--
//       instead we provide an async client only a feature flag.
// See https://github.com/rust-lang/rust/issues/44265
