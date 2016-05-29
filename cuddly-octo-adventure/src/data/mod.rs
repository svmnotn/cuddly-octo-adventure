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
use std::fs::{self, File};
use self::json::ser as to_json;
use self::json::de as from_json;
use self::zip::CompressionMethod;

/// Load a 'cuddle' from disk, a 'cuddle' is a ziped folder.
// TODO Worry about other included files!
pub fn load_cuddle(from: PathBuf) -> Result<Archive> {
    let mut archive = Archive::default();
    let cuddle = fs::File::open(&from)?;
    let mut zip = zip::ZipArchive::new(cuddle)?;

    {
        let mut f = zip.by_name("info")?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        archive.info = from_json::from_str(&data)?;
    }
    {
        let mut f = zip.by_name("settings")?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        archive.settings = from_json::from_str(&data)?;
        // TODO Load Sounds and Background
    }
    let mut topics = String::new();
    {
        let mut f = zip.by_name("topics")?;
        f.read_to_string(&mut topics)?;
    }
    for topic in topics.lines() {
      let mut t = Topic::default();
      t.name = topic.to_owned();

      {
          let mut f = zip.by_name(&format!("{}/{}", topic, "questions"))?;
          let mut data = String::new();
          f.read_to_string(&mut data)?;
          t.questions = from_json::from_str(&data)?;
          // TODO Load images into the questions and answers
      }

      archive.topics.push(t);
    }

    Ok(archive)
}

/// Save a 'cuddle' to disk, a 'cuddle' is a ziped folder.
pub fn save_cuddle(archive: Archive, to: PathBuf) -> Result<()> {
    fn get_name(p: PathBuf) -> String {
        if let Some(name) = p.file_name() {
            if let Ok(n) = name.to_os_string().into_string() {
                return n;
            }
        }
        String::new()
    }

    fn write_loc<'a, W: Write + Seek>(name: &'a str,
                                      opt_loc: Option<PathBuf>,
                                      zip: &mut zip::ZipWriter<W>)
                                      -> Result<()> {
        if let Some(loc) = opt_loc {
            zip.start_file(format!("{}{}", name, get_name(loc.clone())), CompressionMethod::Stored)?;
            let mut f = File::open(loc)?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            zip.write_all(buf.as_slice())?;
        }
        Ok(())
    }

    let cuddle = File::create(to)?;
    let mut zip = zip::ZipWriter::new(cuddle);

    // Save the Archive info into its own json file
    zip.start_file("info", CompressionMethod::Deflated)?;
    zip.write_all(to_json::to_string(&archive.info)?.as_bytes())?;

    // Save the Archive settings into its own json file
    zip.start_file("settings", CompressionMethod::Deflated)?;
    zip.write_all(to_json::to_string(&archive.settings)?.as_bytes())?;

    write_loc("", archive.settings.bg_loc, &mut zip)?;
    write_loc("", archive.settings.sound.bg_sound_loc, &mut zip)?;
    write_loc("", archive.settings.sound.yay_sound_loc, &mut zip)?;
    write_loc("", archive.settings.sound.nay_sound_loc, &mut zip)?;

    let mut topics = String::new();

    for topic in archive.topics {
        topics.push_str(&topic.name);
        // Create a directory per topic
        zip.start_file(format!("{}/", &topic.name), CompressionMethod::Stored)?;

        // Save all the questions in the topic to a json file
        zip.start_file(format!("{}/{}", &topic.name, "questions"), CompressionMethod::Deflated)?;
        zip.write_all(to_json::to_string(&topic.questions)?.as_bytes())?;

        for q in topic.questions {
            write_loc(&format!("{}/", &topic.name), q.img_loc, &mut zip)?;
            for ans in q.answers {
                write_loc(&format!("{}/", &topic.name), ans.img_loc, &mut zip)?;
            }
        }
    }

    zip.start_file("topics", CompressionMethod::Deflated)?;
    zip.write_all(topics.as_bytes())?;
    Ok(())
}
