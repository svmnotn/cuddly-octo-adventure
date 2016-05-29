use super::*;

pub fn build(tabs: &Notebook) {
    let label = Label::new(Some("Settings"));
    let tab = Notebook::new();
    build_settings_core(&tab);
    build_settings_message(&tab);
    build_settings_time(&tab);
    build_settings_sound(&tab);
    tabs.append_page(&tab, Some(&label));
}

fn build_settings_core(tabs: &Notebook) {
    let label = Label::new(Some("Basics"));
    let button = Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("CORE!");
    });
    tabs.append_page(&button, Some(&label));
}

fn build_settings_message(tabs: &Notebook) {
    let label = Label::new(Some("Messages"));
    let button = Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("MESSAGE!");
    });
    tabs.append_page(&button, Some(&label));
}

fn build_settings_sound(tabs: &Notebook) {
    let label = Label::new(Some("Sounds"));
    let button = Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("SOUNDS!");
    });
    tabs.append_page(&button, Some(&label));
}

fn build_settings_time(tabs: &Notebook) {
    let label = Label::new(Some("Time"));
    let button = Button::new_with_label("Click me!");

    button.connect_clicked(|_| {
        println!("TIME!");
    });
    tabs.append_page(&button, Some(&label));
}
