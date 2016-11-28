#![feature(proc_macro)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate tempdir;
extern crate zip;

// TODO The whole thing
pub mod cuddle;
pub mod error;
mod load;
mod save;

pub use load::load_cuddle;
pub use save::save_cuddle;
