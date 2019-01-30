//! API client to work with a remote instance of Home Assistant
//!
//! ```no_run
//! extern crate homeassistant;
//!
//! fn main() {
//!     let client = homeassistant::Client::new("http://192.168.1.2:8123".to_owned(), None);
//!
//!     for response in client.get_states().unwrap() {
//!         let x = client.get_state(&response.entity_id);
//!         println!("{:?}", x);
//!     }
//! }
//! ```
extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod client;
mod error;
pub mod structs;

pub use client::Client;
pub use error::Error;
