// TODO
// * Look into multithreading the load of images into ram.
// * Write Topic in a sensible manner
extern crate tempdir;
extern crate zip;
extern crate serde;
extern crate serde_json as json;

mod answer;
mod archive;
mod question;
mod team;
mod topic;
mod error;
pub mod settings;

pub use self::archive::{Archive, ArchiveInfo};
pub use self::answer::Answer;
pub use self::question::Question;
pub use self::team::Team;
pub use self::topic::Topic;
pub use self::error::{Error, Result};

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::{self, DirEntry, File};
use json::ser as to_json;
use json::de as from_json;

/// Load an 'archive' from disk,
/// an 'archive' is a folder following the structure expected in a 'cuddle'.
/// This is a convinience method.
pub fn load_archive(from: PathBuf) -> Result<Archive> {

/// Load a 'cuddle' from disk, a 'cuddle' is a ziped folder.
pub fn load_cuddle(from: PathBuf) -> Result<Archive> {
    // Unzip cuddle

    // Load cuddle from unziped dir
    // load_archive(cuddle dir)
    unimplemented!()
}

/// Save an 'archive' to disk,
/// an 'archive' is a folder following the structure expected in a 'cuddle'.
/// This is a convinience method.
pub fn save_archive(archive: Archive, to: PathBuf) -> Result<()> {
    {
        // Save the Archive info into its own json file
        let info = to_json::to_string(&archive.info)?;
        to.push("info");
        let mut f = File::create(&to)?;
        f.write_all(&info)?;
        to.pop();
    }
    {
        // Save the Archive settings into its own json file
        let settings = to_json::to_string(&archive.settings)?;
        to.push("settings");
        let mut f = File::create(&to)?;
        f.write_all(&settings)?;
        to.pop();
    }
    for topic in archive.topics {
        // Create a directory per topic
        to.push(&topic.name);
        fs::create_dir(&to)?;
        {
            // Save all the questions in the topic to a json file
            to.push("questions");
            let questions = to_json::to_string(&topic.questions)?;
            let mut f = File::create(&to)?;
            f.write_all(&questions)?;
            to.pop();
        }
        to.pop();
    }
    Ok(())
}

/// Save a 'cuddle' to disk, a 'cuddle' is a ziped folder.
pub fn save_cuddle(archive: Archive, to: PathBuf) -> Result<()> {
    // Save archive to tmp folder
    // save_archive(archive, tmp folder, prob in run dir);

    // Zip contents of tmp folder and move to 'to' dir
    unimplemented!()
}
