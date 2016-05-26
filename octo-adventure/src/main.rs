// Make: Red: 0.1 Green: 0.5 Blue 0.8 Default Background
extern crate coa;
use coa::gtk;
use coa::gtk::prelude::*;
use coa::gtk::{Button, Window, WindowType};

fn main() {
    println!("Hello World! Let's play a game!");
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Cuddly Octo Adventure");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::main();
}
