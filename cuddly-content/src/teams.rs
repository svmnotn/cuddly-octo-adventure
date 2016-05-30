use super::*;

pub fn build(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Teams"));
    let button = gtk::Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("TEAMS!");
    });
    tabs.append_page(&button, Some(&label));
}
