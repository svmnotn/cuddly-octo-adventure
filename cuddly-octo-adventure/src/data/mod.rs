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

use std::path::{Path, PathBuf};
use std::io;
use std::io::prelude::*;
use std::fs::{self, DirEntry, File};
use json::ser as to_json;
use json::de as from_json;
use zip::CompressionMethod;

/// Load an 'archive' from disk,
/// an 'archive' is a folder following the structure expected in a 'cuddle'.
/// This is a convinience method.
pub fn load_archive(from: PathBuf) -> Result<Archive> {
    let mut archive = Archive::default();

    // Load the Archive info from the info json file
    from.push("info");
    let mut f = File::open(&from)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    archive.info = from_json::from_str(&data)?;
    from.pop();

    // Load the Archive settings from the settings json file
    from.push("settings");
    let mut f = File::open(&from)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    archive.settings = from_json::from_str(&data)?;
    from.pop();

    if fs::metadata(&from)?.is_dir() {
        let topics = fs::read_dir(&from)
            .into_iter()
            .map(|e| e?)
            .filter(|e| e.is_dir())
            .collect();
        for topic in topics {
            let mut t = Topic::default();
            t.name = topic.file_name().into_string()?;

            // Load all the questions in the topic from the questions json file
            from.push("questions");
            let mut f = File::open(&from)?;
            let mut data = String::new();
            f.read_to_string(&mut data)?;
            t.questions = from_json::from_str(&data)?;
            from.pop();

        }
    }
    Ok(())
}

/// Load a 'cuddle' from disk, a 'cuddle' is a ziped folder.
pub fn load_cuddle(from: PathBuf) -> Result<Archive> {
    let tmp = tempdir::TempDir::new("cuddle")?;
    // TODO Unzip cuddle into ^

    // Load cuddle from unziped dir
    let dir = PathBuf::new();
    dir.push(tmp.path());
    load_archive(dir)
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
    f.write_all(&to_json::to_string(&archive.info)?)?;
    to.pop();

    // Save the Archive settings into its own json file
        let settings = to_json::to_string(&archive.settings)?;
    to.push("settings");
    let mut f = File::create(&to)?;
    f.write_all(&to_json::to_string(&archive.settings)?)?;
    to.pop();

    for topic in archive.topics {
        // Create a directory per topic
        to.push(&topic.name);
        fs::create_dir(&to)?;

        // Save all the questions in the topic to a json file
        to.push("questions");
        let questions = to_json::to_string(&topic.questions)?;
        let mut f = File::create(&to)?;
        f.write_all(&questions)?;
        to.pop();

        to.pop();
    }
    Ok(())
}

/// Save a 'cuddle' to disk, a 'cuddle' is a ziped folder.
pub fn save_cuddle(archive: Archive, to: PathBuf) -> Result<()> {
    // Save archive to tmp folder
    let tmp = tempdir::TempDir::new("cuddle")?;
    {
        let dir = PathBuf::new();
        dir.push(tmp.path());
        save_archive(archive, dir);
    }

    // TODO Zip contents of tmp folder and move to 'to' dir
    unimplemented!()
}
