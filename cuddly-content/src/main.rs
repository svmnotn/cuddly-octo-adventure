#[macro_use]
extern crate lazy_static;
extern crate coa;

pub use coa::gtk;
pub use coa::gtk::prelude::*;

pub use coa::gdk;
pub use coa::gdk::enums::*;

pub use coa::cuddle::{Answer, Archive, ArchiveInfo, Error as CuddleError, Question, Result as CuddleResult, Team,
                      Topic, settings as cuddle_settings};

pub use std::sync::*;

lazy_static!{
  pub static ref ARCHIVE: RwLock<Archive> = RwLock::new(Archive::default());
}

macro_rules! text_input {
  ($window:ident, $read:expr, $write:expr, $name:expr, $desc:expr) => ({
    let cont = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(Some($name));
    label.set_tooltip_text(Some($desc));

    let entry = gtk::Entry::new();
    entry.set_text(&$read);

    entry.connect_focus_out_event(|e,_| {
      $write = e.get_text().unwrap();
      Inhibit(false)
    });

    cont.add(&label);
    cont.pack_end(&entry, true, true, 5);
    $window.add(&cont);
  })
}

macro_rules! text_input_opt {
  ($window:ident, $read:expr, $write:expr, $name:expr, $desc:expr) => ({
    let cont = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(Some($name));
    label.set_tooltip_text(Some($desc));

    let entry = gtk::Entry::new();

    if let Some(ref txt) = $read {
      entry.set_text(&txt);
    } else {
      entry.set_text("");
    }

    entry.connect_focus_out_event(|e,_| {
      $write = e.get_text();
      Inhibit(false)
    });

    cont.add(&label);
    cont.pack_end(&entry, true, true, 5);
    $window.add(&cont);
  })
}

macro_rules! numeric_input {
  ($window:ident, $read:expr, $write:expr, $name:expr, $desc:expr, $max:expr, $min:expr) => ({
    let cont = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let label = gtk::Label::new(Some($name));
    label.set_tooltip_text(Some($desc));

    let spin = gtk::SpinButton::new_with_range($min, $max, 1f64);
    spin.set_numeric(true);
    spin.set_snap_to_ticks(false);
    spin.set_wrap(true);

    spin.set_value($read);

    spin.connect_focus_out_event(|e,_| {
      $write = e.get_value();
      Inhibit(false)
    });

    cont.add(&label);
    cont.pack_end(&spin, true, true, 5);
    $window.add(&cont);
  })
}

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

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Cuddly Content Creator");
    window.set_default_size(800, 600);

    let holder = gtk::Box::new(gtk::Orientation::Vertical, 0);

    build_menu(&holder);
    build_tabs(&holder);

    window.add(&holder);
    window.show_all();

    window.connect_key_press_event(move |_, key| {
        let keyval = key.as_ref().keyval;
        let keystate = key.as_ref().state;

        if keystate.intersects(modifier_type::ControlMask) && keyval == 115 {
            println!("[Ctrl + s] Saving...");
            open_save();
        }
        if keystate.intersects(modifier_type::ControlMask) && keyval == 111 {
            println!("[Ctrl + o] Opening...");
            open_load();
        }

        Inhibit(false)
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn build_menu(window: &gtk::Box) {
    let menu = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    menu.set_homogeneous(true);

    let save = gtk::Button::new_with_label("Save File As");
    let load = gtk::Button::new_with_label("Load File");

    save.connect_clicked(|_| {
        open_save();
    });
    load.connect_clicked(|_| {
        open_load();
    });

    menu.add(&save);
    menu.add(&load);

    window.add(&menu);
}

fn build_tabs(window: &gtk::Box) {
    let tabs = gtk::Notebook::new();

    build_info(&tabs);
    build_topics(&tabs);
    build_settings(&tabs);
    build_teams(&tabs);

    window.pack_end(&tabs, true, true, 0);
}

fn open_save() {
    let mut a = ARCHIVE.write().unwrap();
    println!("{}", &a.info.name);
    a.info.name = String::from("test!");
    println!("{}", &a.info.name);
}

fn open_load() {
    let mut a = ARCHIVE.write().unwrap();
    println!("{}", &a.info.name);
    a.info.name = String::from("test 2!");
    println!("{}", &a.info.name);
}
