
use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget, Color};
use super::*;

pub struct ArchiveEditor {
    archive: Archive,
    data: TempDir,
    // The `widget::Id` of each widget instantiated in the gui.
    ids: ::support::Ids,
    // The Image Map that will contain all the dynamically loaded pictures
    pub img_map: ::conrod::image::Map<glium::texture::Texture2d>,
}

impl ArchiveEditor {
    pub fn new(a: Archive, d: TempDir, id: ::support::Ids, m: ::conrod::image::Map<glium::texture::Texture2d>) -> Self {
        ArchiveEditor {
            archive: a,
            data: d,
            ids: id,
            img_map: m,
        }
    }

    pub fn gui(&mut self, ui: &mut ::conrod::UiCell) {}
}
