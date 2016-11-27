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
pub use self::topic::Topic;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Load a 'cuddle' from disk, a 'cuddle' is a ziped folder.
/// Any extra files will be extracted into the temporary folder returned,
/// all paths are modified to lead to that folder.
pub fn load_cuddle(from: PathBuf) -> Result<(Archive, TmpDir)> {
    use ::json::de as from_json;

    let mut archive = Archive::default();
    let f = File::open(&from)?;
    let mut cuddle = zip::ZipArchive::new(f)?;
    {
        let mut f = cuddle.by_name("info")?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        archive.info = from_json::from_str(&data)?;
    }
    {
        let mut f = cuddle.by_name("settings")?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        archive.settings = from_json::from_str(&data)?;
    }
    {
        // archive.settings.bg_img = read_img(&mut src, "", archive.settings.bg_loc.as_ref(), &mut zip)?;
        // TODO Load Sounds
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
            t.questions = from_json::from_str::<Vec<Question>>(&data)?;
        }
        // for q in &mut t.questions {
        // {
        // q.img = read_img(&mut src, &format!("{}/", &t.name), q.img_loc.as_ref(), &mut zip)?;
        // }
        // for ans in &mut q.answers {
        // ans.img = read_img(&mut src, &format!("{}/", &t.name), ans.img_loc.as_ref(), &mut zip)?;
        // }
        // }
        archive.topics.push(t);
    }

    Ok(archive)
}

/// Save a 'cuddle' to disk, a 'cuddle' is a ziped folder.
pub fn save_cuddle(archive: Archive, to: PathBuf) -> Result<()> {
    use ::json::ser as to_json;
    use self::zip::write::FileOptions;

    /// Writes a file into the Cuddle
    fn write_file<'a, W: Write + Seek>(name: &'a str,
                                       opt_loc: Option<PathBuf>,
                                       zip: &mut zip::ZipWriter<W>)
                                       -> Result<()> {
        use std::path::Path;

        /// Retrives the extension of a path
        fn get_extension(p: &Path) -> String {
            if let Some(ext) = p.extension() {
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
    write_file("bg", archive.settings.bg_loc, &mut cuddle)?;
    write_file("bg_sound", archive.settings.sound.bg_sound_loc, &mut cuddle)?;
    write_file("yay_sound", archive.settings.sound.yay_sound_loc, &mut cuddle)?;
    write_file("nay_sound", archive.settings.sound.nay_sound_loc, &mut cuddle)?;

    // Go over all the topics in the Archive
    for topic in archive.topics {
        // Create a directory per topic
        cuddle.add_directory(&*topic.name, FileOptions::default())?;

        // Go over all the questions and change their path to refelct them being inside an archive
        for q in topic.questions {
            let s = format!("{}/{}", &topic.name, &q.id);
            write_file(&s, q.img_loc, &mut cuddle)?;

            for (i, ans) in q.answers.into_iter().enumerate() {
                let a = format!("{}-ans-{}", &s, i);
                write_file(&a, ans.img_loc, &mut cuddle)?;
                ans.img_loc = Some(a.into());
            }

            // Change the img_loc of the question after it has been used by the answers
            q.img_loc = Some(s.into());
        }

        // Save all the questions in the topic to a json file
        cuddle.start_file(format!("{}/{}", &topic.name, "questions"), FileOptions::default())?;
        cuddle.write_all(to_json::to_string(&topic.questions)?.as_bytes())?;
    }

    Ok(())
}
