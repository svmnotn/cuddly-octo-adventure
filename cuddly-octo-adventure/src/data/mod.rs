// TODO
// * Look into multithreading the load of images into ram.
// * impl Default to Archive and the rest of its contents.
extern crate serde;
extern crate serde_json;

mod answer;
mod archive;
mod question;
mod team;
mod topic;
pub mod settings;

pub use self::archive::{Archive, ArchiveInfo};
pub use self::answer::Answer;
pub use self::question::Question;
pub use self::team::Team;
pub use self::topic::Topic;

use std::path::PathBuf;

/// Load an 'archive' from disk, an 'archive' is a folder following the structure expected in a 'cuddle'.
/// This is a convinience method.
pub fn load_archive(from: PathBuf) -> Archive {
  unimplemented!()
}

/// Load a 'cuddle' from disk, a 'cuddle' is a ziped folder.
pub fn load_cuddle(from: PathBuf) -> Archive {
  // Unzip cuddle

  // Load cuddle from unziped dir
  // load_archive(cuddle dir)
  unimplemented!()
}

/// Save an 'archive' to disk, an 'archive' is a folder following the structure expected in a 'cuddle'.
/// This is a convinience method.
pub fn save_archive(archive: Archive, to: PathBuf) {
  unimplemented!()
}

/// Save a 'cuddle' to disk, a 'cuddle' is a ziped folder.
pub fn save_cuddle(archive: Archive, to: PathBuf) {
  // Save archive to tmp folder
  // save_archive(archive, tmp folder, prob in run dir);

  // Zip contents of tmp folder and move to 'to' dir
  unimplemented!()
}
