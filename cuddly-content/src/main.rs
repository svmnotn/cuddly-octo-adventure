#[macro_use]
extern crate conrod;
extern crate coa;
extern crate clap;

use coa::TempDir;
use coa::cuddle::{Answer, Archive, ArchiveInfo, Question, Team, Topic, settings as cuddle_settings};
use coa::error::{Result as CuddleResult, Error as CuddleError};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::widget;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

mod app;

fn main() {
    let matches = clap::App::new("Cuddly Content Creator")
        .bin_name("cuddly-content")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Victor M. Suarez <svmnotn@gmail.com>")
        .about("A content creator for the Cuddly Octo Adventure Project.")
        .args_from_usage("[file]             'A cuddle archive to load at the beginning of the program'
                          -d --data=[folder] 'A folder in which to keep all the temporary data'")
        .get_matches();

    let (archive, dir) = if let Some(loc) = matches.value_of("file") {
        coa::load_cuddle(loc, None).expect("Unable to load given cuddle")
    } else {
        use std::time::{UNIX_EPOCH, SystemTime};

        (Archive::default(),
         if let Some(f) = matches.value_of("data") {
                 TempDir::new_in(f,
                                 &format!("coa-new-archive-{:?}",
                                          SystemTime::now()
                                              .duration_since(UNIX_EPOCH)
                                              .expect("Now is before UNIX_EPOCH? Clock is wrong!")
                                              .as_secs()))
             } else {
                 TempDir::new(&format!("coa-new-archive-{:?}",
                                       SystemTime::now()
                                           .duration_since(UNIX_EPOCH)
                                           .expect("Now is before UNIX_EPOCH? Clock is wrong!")
                                           .as_secs()))
             }
             .expect("Unable to create temporary folder"))
    };

    run(archive, dir);
}

fn run(archive: Archive, dir: TempDir) {
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Cuddly Content Creator")
        .build_glium()
        .unwrap();


}
