use super::*;

pub fn build(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Information"));
    let window = gtk::ScrolledWindow::new(None, None);
    let view = gtk::Viewport::new(None, None);
    let cont = gtk::Box::new(gtk::Orientation::Vertical, 10);

    text_input!(cont, ARCHIVE.read().unwrap().info.name, ARCHIVE.write().unwrap().info.name, "Name: ", "The human readable name of the Cuddle.");

    text_input!(cont,
                ARCHIVE.read().unwrap().info.description, ARCHIVE.write().unwrap().info.description,
                "Description: ",
                "The description of the content of this cuddle.\n\
       Ex: Physics questions for all.");

    text_input!(cont,
                ARCHIVE.read().unwrap().info.version,ARCHIVE.write().unwrap().info.version,
                "Version: ",
                "The version of this cuddle.\n\
       This is shown to the users to allow differentiation between the same cuddle.\n\
       Ex: 1.0.0");

    text_input!(cont,
                ARCHIVE.read().unwrap().info.author,ARCHIVE.write().unwrap().info.author,
                "Author: ",
                "The human that made this cuddle and preferibly where to contact them.\n\
      Ex: Sir Issac Newton <issacnewton@email.com>");

    text_input!(cont,
                ARCHIVE.read().unwrap().info.license,ARCHIVE.write().unwrap().info.license,
                "License: ",
                "The name of the license under which this cuddle is created.");

    text_input!(cont,
                ARCHIVE.read().unwrap().info.update_url,ARCHIVE.write().unwrap().info.update_url,
                "Update URL: ",
                "Where to find updates for this Cuddle.");

    view.add(&cont);
    window.add(&view);
    tabs.append_page(&window, Some(&label));
}
