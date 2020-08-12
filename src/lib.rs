#![deny(missing_docs)]

//! Unofficial Rust library for the SendGrid API.
//! 
//! This crate requires Rust 1.15 or higher as it uses a crate that has a custom derive
//! implementation.
//! 
//! sendgrid-rs implements all of the functionality of other supported SendGrid client libraries.
//! To use sendgrid-rs you must first create a SendGrid account and generate an API key. To create
//! an API key for your SendGrid account, use the account management interface or see the
//! [SendGrid API Documentation](https://sendgrid.com/docs/API_Reference/Web_API_v3/API_Keys/index.html).
//! 
//! sendgrid-rs is available on [crates.io](https://crates.io/crates/sendgrid) and can be included
//! in your Cargo.toml as follows:
//!
//! ```toml
//! [dependencies]
//! sendgrid = "X.X.X"
//! ```
//!
//! # Features
//! The projects has the following feature flags:
//! * `rustls`: this feature flag switches the default SSL provider (OpenSSL) with RusTLS, which is
//! an OpenSSL reimplementation in Rust.
//! * `async`: this feature flag changes the `send` function on the `SGClient` into an `async fn`.
//! Note that without this feature flag, constructing an `SGClient` inside an `async fn` is not
//! possible.
//! 
//! ## Build Dependencies
//! This library utilises [reqwest](https://crates.io/crates/reqwest). Follow the instructions on
//! the [reqwest README](https://github.com/seanmonstar/reqwest#requirements) in order to enable
//! sending HTTPS requests to the SendGrid API.
//! 
//! ## Features
//! You can take advantage of a couple features for the crate. To enable the asynchronous send
//! function, you can use the `async` flag. To enable the [rustls](https://github.com/ctz/rustls)
//! TLS feature, use the `rustls` flag.
//! 
//! ## Example
//! An example of using this library can be found in the examples directory. This example code
//! expects to find your SendGrid API key in the process environment. In shells such as Bash or ZSH
//! this can be set as follows:
//! 
//! ```shell
//! export SENDGRID_API_KEY="SG.my.api.key"
//! ```
//! 
//! ## Documentation
//! [Documentation](https://docs.rs/sendgrid)
//! 
//! Please don't hesitate to contact me at the email listed in my profile. I will try to help as
//! quickly as I can. If you would like to contribute, contact me as well.
//! 
//! ## Mentions
//! Thanks to [meehow](https://github.com/meehow) for their contributions.
//! 
//! Thanks to [richo](https://github.com/richo) for their improvements to the V2 API.
//! 
//! ## License
//! MIT

/// Contains the error type used in this library.
pub mod errors;
mod mail;
mod sg_client;
pub mod v3;

pub use mail::{Destination, Mail};
pub use sg_client::SGClient;
pub use errors::{SendgridError, SendgridResult};
