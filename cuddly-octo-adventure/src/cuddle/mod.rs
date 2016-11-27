extern crate tempdir;
extern crate zip;

mod answer;
mod archive;
mod question;
mod team;
mod topic;
mod error;
pub mod settings;

pub use self::answer::Answer;
pub use self::archive::{Archive, ArchiveInfo};
pub use self::error::{Error, Result};
pub use self::question::Question;
pub use self::team::Team;
use self::tempdir::TempDir;
pub use self::topic::Topic;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Load a 'cuddle' from disk, a 'cuddle' is a ziped folder.
/// Any extra files will be extracted into the temporary folder returned,
/// all paths are modified to lead to that folder.
pub fn load_cuddle<P: AsRef<Path>>(from: P, folder: Option<P>) -> Result<(Archive, TempDir)> {
    use ::json::de as from_json;
    use self::zip::read::ZipFile;

    fn write_file<P: AsRef<Path>>(folder: P, file: ZipFile) -> Result<()> {
        // DO THINGS!
        Ok(())
    }

    let mut archive = Archive::default();

    let f = File::open(from)?;
    let mut cuddle = zip::ZipArchive::new(f)?;

    {
        // Load the Info section of the cuddle
        let mut f = cuddle.by_name("info")?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        archive.info = from_json::from_str(&data)?;
    }

    let dir_name = &format!("coa-{}-decompressed", &archive.info.name);
    let dir = if let Some(p) = folder {
            TempDir::new_in(p, dir_name)
        } else {
            TempDir::new(dir_name)
        }
        .expect("Creating decompress directory");

    {
        // Load the Settings section of the cuddle
        let mut f = cuddle.by_name("settings")?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        archive.settings = from_json::from_str(&data)?;
    }

    archive.topics = vec![];

    // Iterate over every other file
    for i in 2..cuddle.len() {
        let mut f = cuddle.by_index(i)?;
        println!("{}:{} - {:?}", i, f.name(), f.compression());
        let name = String::from(f.name());

        if name.contains("bg_img") {
            // This is the bg image, decompress it to the folder
            write_file(dir.path(), f)?;
            // Update the path in the Archive
        } else if name.contains("bg_sound") {
            // This is the bg sound, decompress it to the folder
            write_file(dir.path(), f)?;
            // Update the path in the Archive
        } else if name.contains("yay_sound") {
            // This is the yay sound, decompress it to the folder
            write_file(dir.path(), f)?;
            // Update the path in the Archive
        } else if name.contains("nay_sound") {
            // This is the nay sound, decompress it to the folder
            write_file(dir.path(), f)?;
            // Update the path in the Archive
        } else {
            use self::zip::CompressionMethod;
            // This is a Topic or is within a topic
            match f.compression() {
                CompressionMethod::Deflated => {
                    // The file is compressed (not a folder)
                    if let Some(n) = name.split('/').nth(0) {
                        // It has a valid name
                        if name.contains("questions") {
                            // Here we have the questions file, so lets load it
                            let mut t = Topic::default();
                            t.name = n.into();
                            let mut data = String::with_capacity(f.size() as usize);
                            f.read_to_string(&mut data)?;
                            t.questions = from_json::from_str::<Vec<Question>>(&data)?;
                            archive.topics.push(t);
                        } else if let Some(id) = name.split('/').nth(1) {
                            // We got a picture
                            if id.contains("ans") {
                                // It is an answer
                                write_file(dir.path(), f)?;
                                // Update the path in the Archive
                            } else {
                                // It is a question
                                write_file(dir.path(), f)?;
                                // Update the path in the Archive
                            }
                        }
                    }
                }
                _ => {}// Not compressed.
            }
        }
    }

    Ok((archive, dir))
}

/// Save a 'cuddle' to disk, a 'cuddle' is a ziped folder.
pub fn save_cuddle<P: AsRef<Path>>(archive: Archive, to: P) -> Result<()> {
    use ::json::ser as to_json;
    use self::zip::write::FileOptions;

    /// Writes a file into the Cuddle
    fn write_file<'a, W: Write + Seek, P: AsRef<Path>>(name: &'a str,
                                                       opt_loc: Option<P>,
                                                       zip: &mut zip::ZipWriter<W>)
                                                       -> Result<()> {
        /// Retrives the extension of a path
        fn get_extension<P: AsRef<Path>>(p: P) -> String {
            if let Some(ext) = p.as_ref().extension() {
                if let Ok(s) = ext.to_os_string().into_string() {
                    return s;
                }
            }
            String::new()
        }

        if let Some(loc) = opt_loc {
            zip.start_file(format!("{}.{}", name, get_extension(&loc)), FileOptions::default())?;
            let mut f = File::open(loc)?;
            let mut buf = Vec::with_capacity(f.metadata()?.len() as usize);
            f.read_to_end(&mut buf)?;
            zip.write_all(buf.as_slice())?;
        }
        Ok(())
    }

    let f = File::create(to)?;
    let mut cuddle = zip::ZipWriter::new(f);

    // Save the Archive info into its own json file
    cuddle.start_file("info", FileOptions::default())?;
    cuddle.write_all(to_json::to_string(&archive.info)?.as_bytes())?;

    // Save the Archive settings into its own json file
    cuddle.start_file("settings", FileOptions::default())?;
    cuddle.write_all(to_json::to_string(&archive.settings)?.as_bytes())?;

    // Save the extra files to the Cuddle
    write_file("bg_img", archive.settings.bg_loc, &mut cuddle)?;
    write_file("bg_sound", archive.settings.sound.bg_sound_loc, &mut cuddle)?;
    write_file("yay_sound", archive.settings.sound.yay_sound_loc, &mut cuddle)?;
    write_file("nay_sound", archive.settings.sound.nay_sound_loc, &mut cuddle)?;

    // Go over all the topics in the Archive
    for topic in archive.topics {
        // Create a directory per topic
        cuddle.add_directory(&*topic.name, FileOptions::default())?;

        // Save all the questions in the topic to a json file
        cuddle.start_file(format!("{}/{}", topic.name, "questions"), FileOptions::default())?;
        cuddle.write_all(to_json::to_string(&topic.questions)?.as_bytes())?;

        // Go over all the questions and change their path to refelct them being inside an archive
        for q in &topic.questions {
            let s = format!("{}/{}", &topic.name, &q.id);

            write_file(&s, q.img_loc.as_ref(), &mut cuddle)?;

            for (i, ans) in q.answers.iter().enumerate() {
                let a = format!("{}-ans-{}", &s, i);
                write_file(&a, ans.img_loc.as_ref(), &mut cuddle)?;
            }
        }
    }

    Ok(())
}
