use super::*;

pub fn build(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Teams"));
    let window = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let scroll = gtk::ScrolledWindow::new(None, None);
    let view = gtk::Viewport::new(None, None);
    let cont = gtk::Box::new(gtk::Orientation::Vertical, 0);

    for i in 0..ARCHIVE.read().unwrap().settings.teams.len() {
      build_team(i, &cont);
    }

    cont.connect_draw(|c, _| {
      let children = c.get_children();
      let teams = &ARCHIVE.read().unwrap().settings.teams;

      if children.len() != teams.len() {
        for child in children {
            c.remove(&child);
        }
        for i in 0..teams.len() {
            build_team(i, &c);
        }
        c.show_all();
      }
      Inhibit(false)
    });

    view.add(&cont);
    scroll.add(&view);
    window.pack_start(&scroll, true, true, 0);

    let edit = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    let add = gtk::Button::new_with_label("Add Team");
    let rem = gtk::Button::new_with_label("Remove Last Team");

    add.connect_clicked(|_| {
        ARCHIVE.write().unwrap().settings.teams.push(Team::default());
    });
    rem.connect_clicked(|_| {
        ARCHIVE.write().unwrap().settings.teams.pop();
    });

    edit.pack_start(&add, true, true, 10);
    edit.pack_end(&rem, true, true, 10);

    window.pack_end(&edit, false, false, 2);
    tabs.append_page(&window, Some(&label));
}

fn build_team(team_index: usize, window: &gtk::Box) {
  let cont = gtk::Frame::new(Some("Team"));
  text_input!(cont,
    ARCHIVE.read().unwrap().settings.teams[team_index].name,
    ARCHIVE.write().unwrap().settings.teams[team_index].name,
    "Team Name: ", "The name that will be shown to identify the team");
  window.add(&cont);
}
