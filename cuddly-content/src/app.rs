use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Color};
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
        widget::Canvas::new().pad(15.0).set(self.ids.main_canvas, ui);

        // Setup the Save and Load buttons
        self.make_sl_buttons(ui);

        // Setup the tabs
        self.make_tabs(ui);
    }

    fn make_sl_buttons(&mut self, ui: &mut ::conrod::UiCell) {
        widget::Canvas::new()
            .mid_top_of(self.ids.main_canvas)
            .kid_area_w_of(self.ids.main_canvas)
            .h(60.0)
            .color(color::TRANSPARENT)
            .set(self.ids.menu_canvas, ui);

        for _press in widget::Button::new()
            .label("Load Cuddle")
            .mid_right_of(self.ids.menu_canvas)
            .w((ui.kid_area_of(self.ids.menu_canvas).unwrap().w() * 0.5) - 10.0)
            .set(self.ids.load_btn, ui) {
            // TODO: Ask where to get the archive from!
            let from = "./test.cuddle";
            let (a, d) = ::coa::load_cuddle(from, Some(&self.dir)).expect("Unable to load cuddle");
            self.archive = a;
            self.data = d;
        }

        for _press in widget::Button::new()
            .label("Save Cuddle")
            .mid_left_of(self.ids.menu_canvas)
            .w((ui.kid_area_of(self.ids.menu_canvas).unwrap().w() * 0.5) - 10.0)
            .set(self.ids.save_btn, ui) {
            // TODO: Ask for place to store this archive
            let to = "./test.cuddle";
            ::coa::save_cuddle(&self.archive, to);
        }
    }

    fn make_tabs(&mut self, ui: &mut ::conrod::UiCell) {}
}
