#![feature(custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]
// TODO The whole thing
extern crate gtk as gtk_rs;
extern crate gdk as gdk_rs;

pub mod gtk {
  pub use gtk_rs::*;
}

pub mod gdk {
  pub use gdk_rs::*;
}

pub mod cuddle;
