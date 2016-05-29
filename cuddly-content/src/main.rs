extern crate coa;

use coa::gtk::{Window, WindowType};
pub use coa::cuddle::{
  Archive,
  ArchiveInfo,
  Answer,
  Question,
  Team,
  Topic,
  Error as CuddleError,
  Result as CuddleResult,
  settings as cuddle_settings,
};
pub use coa::gtk::prelude::*;
pub use coa::gtk::{
  self,
  Button, // Buttons! :D
  Entry, // Text Input
  Label, // Show all the text!
  Notebook, // TABS!
};

mod info;
use info::build as build_info;

mod topics;
use topics::build as build_topics;

mod settings;
use settings::build as build_settings;

mod teams;
use teams::build as build_teams;

fn main() {
    println!("Hello World! Let's make some content!");
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Cuddly Content Creator");
    window.set_default_size(800, 600);
    let tabs = Notebook::new();

    build_info(&tabs);
    build_topics(&tabs);
    build_settings(&tabs);
    build_teams(&tabs);

    window.add(&tabs);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
