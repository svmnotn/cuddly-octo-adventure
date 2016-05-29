use super::*;

pub fn build(tabs: &Notebook) {
    let label = Label::new(Some("Information"));
    let button = Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("INFO!");
    });
    tabs.append_page(&button, Some(&label));
}
