use super::*;

pub fn build(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Topics"));
    let button = gtk::Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("TOPICS!");
    });
    tabs.append_page(&button, Some(&label));
}
