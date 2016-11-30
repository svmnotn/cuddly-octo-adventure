use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget, Color};
use std::path::{Path, PathBuf};
use super::*;

pub struct ArchiveEditor {
    archive: Archive,
    dir: PathBuf,
    data: TempDir,
    // The `widget::Id` of each widget instantiated in the gui.
    ids: ::support::Ids,
    // The Image Map that will contain all the dynamically loaded pictures
    pub img_map: ::conrod::image::Map<glium::texture::Texture2d>,
}

#[allow(dead_code)]
const IMAGE_EXTENSIONS: [&'static str; 3] = ["jpeg", "jpg", "png"];
#[allow(dead_code)]
const SOUND_EXTENSIONS: [&'static str; 2] = ["wav", "mp3"];

impl ArchiveEditor {
    pub fn new<P: Into<PathBuf>>(a: Archive,
                                 path: P,
                                 d: TempDir,
                                 id: ::support::Ids,
                                 m: ::conrod::image::Map<glium::texture::Texture2d>)
                                 -> Self {
        ArchiveEditor {
            archive: a,
            dir: path.into(),
            data: d,
            ids: id,
            img_map: m,
        }
    }

    pub fn gui(&mut self, ui: &mut ::conrod::UiCell) {
        // Set up the main canvas
        widget::Canvas::new()
            .pad(10.0)
            .no_parent()
            .color(color::TRANSPARENT)
            .flow_down(&[(self.ids.menu_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.main_canvas)
                              .color(color::TRANSPARENT)
                              .length_weight(0.1)
                              .flow_right(&[(self.ids.load_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.menu_canvas)
                                                 .color(color::TRANSPARENT)
                                                 .length_weight(0.5)),
                                            (self.ids.save_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.menu_canvas)
                                                 .color(color::TRANSPARENT)
                                                 .length_weight(0.5))])),
                         (self.ids.tabs_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.main_canvas)
                              .color(color::TRANSPARENT)
                              .length_weight(0.9))])
            .set(self.ids.main_canvas, ui);

        // Setup the Save and Load buttons
        self.make_sl_buttons(ui);

        // Setup the tabs
        self.make_tabs(ui);
    }

    /// Set up the Save and Load Buttons
    fn make_sl_buttons(&mut self, ui: &mut ::conrod::UiCell) {
        for _press in widget::Button::new()
            .label("Load Cuddle")
            .parent(self.ids.load_canvas)
            .middle_of(self.ids.load_canvas)
            .padded_wh_of(self.ids.load_canvas, 10.0)
            .set(self.ids.load_btn, ui) {
            // TODO: Ask where to get the archive from!
            let from = "./test.cuddle";
            // Load the given cuddle and modify our data
            let (a, d) = ::coa::load_cuddle(from, Some(&self.dir)).expect("Unable to load cuddle");
            self.archive = a;
            self.data = d;
        }

        for _press in widget::Button::new()
            .label("Save Cuddle")
            .parent(self.ids.save_canvas)
            .middle_of(self.ids.save_canvas)
            .padded_wh_of(self.ids.save_canvas, 10.0)
            .set(self.ids.save_btn, ui) {
            // TODO: Ask for place to store this archive
            let to = "./test.cuddle";
            // Save our data to disk
            println!("{:?}", &self.archive);
            //::coa::save_cuddle(&self.archive, to);
        }
    }

    /// Setup the Tabs Container
    fn make_tabs(&mut self, ui: &mut ::conrod::UiCell) {
        widget::Tabs::new(&[(self.ids.info_tab, "Information"),
                            (self.ids.topics_tab, "Topics"),
                            (self.ids.teams_tab, "Teams"),
                            (self.ids.settings_tab, "Settings")])
            .layout_horizontally()
            .parent(self.ids.tabs_canvas)
            .bar_thickness(36.0)
            .starting_canvas(self.ids.info_tab)
            .middle_of(self.ids.tabs_canvas)
            .padded_wh_of(self.ids.tabs_canvas, 10.0)
            .border(2.0)
            .border_color(color::BLACK)
            .set(self.ids.tabs, ui);

        // Setup the Information Tab
        self.make_info_tab(ui);

        // Setup the Topics Tab
        self.make_topics_tab(ui);

        // Setup the Teams Tab
        self.make_teams_tab(ui);

        // Setup the Settings Tab
        self.make_settings_tab(ui);
    }

    /// Setup the Information Tab
    fn make_info_tab(&mut self, ui: &mut ::conrod::UiCell) {
        macro_rules! info_canvas {
          ($c:ident, $l:ident, $i:ident) => {(self.ids.$c,
                        widget::Canvas::new()
                            .parent(self.ids.info_tab)
                            .color(color::TRANSPARENT)
                            .length_weight(1.0 / 6.0)
                            .flow_right(&[(self.ids.$l,
                                           widget::Canvas::new()
                                               .parent(self.ids.$c)
                                               .color(color::TRANSPARENT)
                                               .length_weight(0.2)),
                                          (self.ids.$i,
                                           widget::Canvas::new()
                                               .parent(self.ids.$c)
                                               .color(color::TRANSPARENT)
                                               .length_weight(0.8))]))};
        }

        widget::Canvas::new()
            .parent(self.ids.info_tab)
            .color(color::TRANSPARENT)
            .middle_of(self.ids.info_tab)
            .padded_wh_of(self.ids.info_tab, 10.0)
            .flow_down(&[info_canvas!(info_name_canvas, info_name_label_canvas, info_name_input_canvas),
                         info_canvas!(info_desc_canvas, info_desc_label_canvas, info_desc_input_canvas),
                         info_canvas!(info_ver_canvas, info_ver_label_canvas, info_ver_input_canvas),
                         info_canvas!(info_author_canvas, info_author_label_canvas, info_author_input_canvas),
                         info_canvas!(info_license_canvas, info_license_label_canvas, info_license_input_canvas),
                         info_canvas!(info_update_canvas, info_update_label_canvas, info_update_input_canvas)])
            .set(self.ids.info_canvas, ui);

        macro_rules! info_part {
              ($title:expr, $a:ident, $lc:ident, $l:ident, $ic:ident, $i:ident) => {
                widget::Text::new($title)
                    .parent(self.ids.$lc)
                    .middle_of(self.ids.$lc)
                    .padded_wh_of(self.ids.$lc, 5.0)
                    .align_text_left()
                    .set(self.ids.$l, ui);

                for string in widget::TextEdit::new(&self.archive.info.$a)
                    .parent(self.ids.$ic)
                    .middle_of(self.ids.$ic)
                    .padded_wh_of(self.ids.$ic, 5.0)
                    .align_text_left()
                    .set(self.ids.$i, ui) {
                    self.archive.info.$a = string;
                }
              };
            }

        // Setup the Name
        info_part!("Cuddle Name:",
                   name,
                   info_name_label_canvas,
                   info_name_label,
                   info_name_input_canvas,
                   info_name_input);

        // Setup the Description
        info_part!("Cuddle Description:",
                   description,
                   info_desc_label_canvas,
                   info_desc_label,
                   info_desc_input_canvas,
                   info_desc_input);

        // Setup the Version
        info_part!("Cuddle Version:",
                   version,
                   info_ver_label_canvas,
                   info_ver_label,
                   info_ver_input_canvas,
                   info_ver_input);

        // Setup the Author
        info_part!("Cuddle Author:",
                   author,
                   info_author_label_canvas,
                   info_author_label,
                   info_author_input_canvas,
                   info_author_input);

        // Setup the License
        info_part!("Cuddle License:",
                   license,
                   info_license_label_canvas,
                   info_license_label,
                   info_license_input_canvas,
                   info_license_input);

        // Setup the Update URl
        info_part!("Cuddle Update URL:",
                   update_url,
                   info_update_label_canvas,
                   info_update_label,
                   info_update_input_canvas,
                   info_update_input);
    }

    /// Setup the Topics Tab
    fn make_topics_tab(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Teams Tab
    fn make_teams_tab(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Settings Tab
    fn make_settings_tab(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Creates a File Picker
    fn make_file_picker<P: AsRef<Path>>(&mut self, label: &str, start: P, ext: &[&str], ui: &mut ::conrod::UiCell) {
        widget::Canvas::new()
            .no_parent()
            .floating(true)
            .color(color::DARK_CHARCOAL)
            .label(label)
            .label_font_size(20)
            .border(1.0)
            .border_color(color::BLACK)
            .w(800.0)
            .h(600.0)
            .middle_of(self.ids.main_canvas)
            .set(self.ids.file_canvas, ui);

        for e in widget::FileNavigator::with_extension(start.as_ref(), ext)
            .parent(self.ids.file_canvas)
            .color(color::TRANSPARENT)
            .unselected_color(color::TRANSPARENT)
            .middle_of(self.ids.file_canvas)
            .padded_wh_of(self.ids.file_canvas, 20.0)
            .set(self.ids.file_navigator, ui) {
            match e {
                widget::file_navigator::Event::DoubleClick(_, vec) => {
                    println!("{:?}", vec);
                }
                _ => {}
            }
        }
    }
}
