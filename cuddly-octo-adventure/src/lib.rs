// TODO Add Documentation.
#![cfg_attr(feature = "serde_derive", feature(proc_macro))]
#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate tempdir;
extern crate zip;

pub mod cuddle;
pub mod error;
mod load;
mod save;

pub use load::load_cuddle;
pub use save::save_cuddle;
pub use tempdir::TempDir;
