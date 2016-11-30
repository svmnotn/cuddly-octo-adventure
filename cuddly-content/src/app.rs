use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget, Color};
use std::path::PathBuf;
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
            ::coa::save_cuddle(&self.archive, to);
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
        widget::Canvas::new()
            .parent(self.ids.info_tab)
            .color(color::TRANSPARENT)
            .middle_of(self.ids.info_tab)
            .padded_wh_of(self.ids.info_tab, 10.0)
            .flow_down(&[(self.ids.info_name_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.info_tab)
                              .color(color::TRANSPARENT)
                              .length_weight(1.0 / 6.0)
                              .flow_right(&[(self.ids.info_name_label_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_name_canvas)
                                                 .color(color::RED)
                                                 .length_weight(0.2)),
                                            (self.ids.info_name_input_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_name_canvas)
                                                 .color(color::BLUE)
                                                 .length_weight(0.8))])),
                         (self.ids.info_desc_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.info_tab)
                              .color(color::TRANSPARENT)
                              .length_weight(1.0 / 6.0)
                              .flow_right(&[(self.ids.info_desc_label_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_desc_canvas)
                                                 .color(color::BLUE)
                                                 .length_weight(0.2)),
                                            (self.ids.info_desc_input_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_desc_canvas)
                                                 .color(color::RED)
                                                 .length_weight(0.8))])),
                         (self.ids.info_ver_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.info_tab)
                              .color(color::TRANSPARENT)
                              .length_weight(1.0 / 6.0)
                              .flow_right(&[(self.ids.info_ver_label_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_ver_canvas)
                                                 .color(color::RED)
                                                 .length_weight(0.2)),
                                            (self.ids.info_ver_input_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_ver_canvas)
                                                 .color(color::BLUE)
                                                 .length_weight(0.8))])),
                         (self.ids.info_author_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.info_tab)
                              .color(color::TRANSPARENT)
                              .length_weight(1.0 / 6.0)
                              .flow_right(&[(self.ids.info_author_label_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_author_canvas)
                                                 .color(color::BLUE)
                                                 .length_weight(0.2)),
                                            (self.ids.info_author_input_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_author_canvas)
                                                 .color(color::RED)
                                                 .length_weight(0.8))])),
                         (self.ids.info_license_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.info_tab)
                              .color(color::TRANSPARENT)
                              .length_weight(1.0 / 6.0)
                              .flow_right(&[(self.ids.info_license_label_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_license_canvas)
                                                 .color(color::RED)
                                                 .length_weight(0.2)),
                                            (self.ids.info_license_input_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_license_canvas)
                                                 .color(color::BLUE)
                                                 .length_weight(0.8))])),
                         (self.ids.info_update_canvas,
                          widget::Canvas::new()
                              .parent(self.ids.info_tab)
                              .color(color::TRANSPARENT)
                              .length_weight(1.0 / 6.0)
                              .flow_right(&[(self.ids.info_update_label_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_update_canvas)
                                                 .color(color::BLUE)
                                                 .length_weight(0.2)),
                                            (self.ids.info_update_input_canvas,
                                             widget::Canvas::new()
                                                 .parent(self.ids.info_update_canvas)
                                                 .color(color::RED)
                                                 .length_weight(0.8))]))])
            .set(self.ids.info_canvas, ui);

        // Setup the Name
        self.make_info_name(ui);
        // Setup the Description
        self.make_info_desc(ui);
        // Setup the Version
        self.make_info_ver(ui);
        // Setup the Author
        self.make_info_author(ui);
        // Setup the License
        self.make_info_license(ui);
        // Setup the Update URl
        self.make_info_update(ui);
    }

    /// Setup the Name part of the Information Tab
    fn make_info_name(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Name part of the Information Tab
    fn make_info_desc(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Version part of the Information Tab
    fn make_info_ver(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Author part of the Information Tab
    fn make_info_author(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the License part of the Information Tab
    fn make_info_license(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Update URL part of the Information Tab
    fn make_info_update(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Topics Tab
    fn make_topics_tab(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Teams Tab
    fn make_teams_tab(&mut self, ui: &mut ::conrod::UiCell) {}

    /// Setup the Settings Tab
    fn make_settings_tab(&mut self, ui: &mut ::conrod::UiCell) {}
}
