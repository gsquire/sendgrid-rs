pub mod errors;
mod mail;
mod sg_client;
pub mod v3;

pub use crate::mail::{Destination, Mail};
pub use crate::sg_client::SGClient;
