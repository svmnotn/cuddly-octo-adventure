use super::*;

pub fn build(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Settings"));
    let tab = gtk::Notebook::new();

    build_settings_core(&tab);
    build_settings_message(&tab);
    build_settings_time(&tab);
    build_settings_sound(&tab);

    tabs.append_page(&tab, Some(&label));
}

fn build_settings_core(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Basics"));
    let window = gtk::ScrolledWindow::new(None, None);
    let view = gtk::Viewport::new(None, None);
    let cont = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // TODO add bg loc chooser and colors

    view.add(&cont);
    window.add(&view);
    tabs.append_page(&window, Some(&label));
}

fn build_settings_message(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Messages"));
    let window = gtk::ScrolledWindow::new(None, None);
    let view = gtk::Viewport::new(None, None);
    let cont = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // Score
    text_input!(cont,
                ARCHIVE.read().unwrap().settings.message.score_presentation,
                ARCHIVE.write().unwrap().settings.message.score_presentation,
                "Score Presentation: ",
                "How the score is presented to the players.");

    // Game Over
    text_input!(cont,
                ARCHIVE.read().unwrap().settings.message.game_over,
                ARCHIVE.write().unwrap().settings.message.game_over,
                "Game Over: ",
                "The message to be displayed on game over.");

    // Team Prefix
    text_input!(cont,
                ARCHIVE.read().unwrap().settings.message.team_prefix,
                ARCHIVE.write().unwrap().settings.message.team_prefix,
                "Team Prefix: ",
                "Something written before the team's name is presented.");

    // Score Prefix
    text_input!(cont,
                ARCHIVE.read().unwrap().settings.message.score_prefix,
                ARCHIVE.write().unwrap().settings.message.score_prefix,
                "Score Prefix: ",
                "Something written before the score is presented.");

    // Singular Point Name
    text_input!(cont,
                ARCHIVE.read().unwrap().settings.message.singular_point_name,
                ARCHIVE.write().unwrap().settings.message.singular_point_name,
                "Singular Point Name: ",
                "The name of a single point.");

    // Plural Point Name
    text_input_opt!(cont,
                    ARCHIVE.read().unwrap().settings.message.plural_point_name,
                    ARCHIVE.write().unwrap().settings.message.plural_point_name,
                    "Plural Point Name: ",
                    "The name of more than one point.\n\
    If left blank the singluar name will be used and an 's' will be added at the end.");

    view.add(&cont);
    window.add(&view);
    tabs.append_page(&window, Some(&label));
}

fn build_settings_sound(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Sounds"));
    let window = gtk::ScrolledWindow::new(None, None);
    let view = gtk::Viewport::new(None, None);
    let cont = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // bg_vol
    numeric_input!(cont,
                   ARCHIVE.read().unwrap().settings.sound.bg_sound_vol,
                   ARCHIVE.write().unwrap().settings.sound.bg_sound_vol,
                   "Default Background Music Volume: ",
                   "The default volume of the background music.",
                   100f64,
                   0f64);

    // yay_sound_vol
    numeric_input!(cont,
                   ARCHIVE.read().unwrap().settings.sound.yay_sound_vol,
                   ARCHIVE.write().unwrap().settings.sound.yay_sound_vol,
                   "Default Yay Sound Volume: ",
                   "The default volume of the sound played when an answer is correct.",
                   100f64,
                   0f64);

    // nay_sound_vol
    numeric_input!(cont,
                   ARCHIVE.read().unwrap().settings.sound.nay_sound_vol,
                   ARCHIVE.write().unwrap().settings.sound.nay_sound_vol,
                   "Default Nay Sound Volume: ",
                   "The default volume of the sound played when an answer is wrong.",
                   100f64,
                   0f64);

    view.add(&cont);
    window.add(&view);
    tabs.append_page(&window, Some(&label));
}

fn build_settings_time(tabs: &gtk::Notebook) {
    let label = gtk::Label::new(Some("Time"));
    let window = gtk::ScrolledWindow::new(None, None);
    let view = gtk::Viewport::new(None, None);
    let cont = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // pre_ans_delay
    numeric_input!(cont,
                   ARCHIVE.read().unwrap().settings.time.pre_ans_delay,
                   ARCHIVE.write().unwrap().settings.time.pre_ans_delay,
                   "Pre Answer Delay: ",
                   "The amount of seconds to wait for an answer to be given.",
                   3600f64,
                   0f64);

    // post_ans_delay
    numeric_input!(cont,
                   ARCHIVE.read().unwrap().settings.time.post_ans_delay,
                   ARCHIVE.write().unwrap().settings.time.post_ans_delay,
                   "Post Answer Delay: ",
                   "The amount of sconds to wait after an answer has been given.",
                   3600f64,
                   0f64);

    // score_delay
    numeric_input!(cont,
                   ARCHIVE.read().unwrap().settings.time.score_delay,
                   ARCHIVE.write().unwrap().settings.time.score_delay,
                   "Score Delay: ",
                   "The amount of sconds to wait while displaying the scores.",
                   3600f64,
                   0f64);

    // game_over_delay
    numeric_input!(cont,
                   ARCHIVE.read().unwrap().settings.time.game_over_delay,
                   ARCHIVE.write().unwrap().settings.time.game_over_delay,
                   "Game Over Delay: ",
                   "The amount of time to wait while displaying the game over screen.",
                   3600f64,
                   0f64);

    view.add(&cont);
    window.add(&view);
    tabs.append_page(&window, Some(&label));
}
