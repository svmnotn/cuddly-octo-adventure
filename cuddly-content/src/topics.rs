use super::*;

pub fn build(tabs: &Notebook) {
    let label = Label::new(Some("Topics"));
    let button = Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("TOPICS!");
    });
    tabs.append_page(&button, Some(&label));
}
