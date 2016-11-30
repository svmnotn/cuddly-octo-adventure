use ::cuddle::Archive;
use ::error::Result;
use json::ser as to_json;
use std::fs;
use std::io::{Seek, Write, Read};
use std::path::Path;
use zip::write::{FileOptions, ZipWriter};

/// Save a 'cuddle' to disk, a 'cuddle' is a ziped folder.
pub fn save_cuddle<P: AsRef<Path>>(archive: &Archive, to: P) -> Result<()> {
    let f = fs::File::create(to)?;
    let mut cuddle = ZipWriter::new(f);

    // Save the Archive info into its own json file
    cuddle.start_file("info", FileOptions::default())?;
    cuddle.write_all(to_json::to_string(&archive.info)?.as_bytes())?;

    // Save the Archive settings into its own json file
    cuddle.start_file("settings", FileOptions::default())?;
    cuddle.write_all(to_json::to_string(&archive.settings)?.as_bytes())?;

    // Save the extra files to the Cuddle
    write_file("bg_img", archive.settings.bg_loc.as_ref(), &mut cuddle)?;
    write_file("bg_sound", archive.settings.sound.bg_sound_loc.as_ref(), &mut cuddle)?;
    write_file("yay_sound", archive.settings.sound.yay_sound_loc.as_ref(), &mut cuddle)?;
    write_file("nay_sound", archive.settings.sound.nay_sound_loc.as_ref(), &mut cuddle)?;

    // Go over all the topics in the Archive
    for topic in &archive.topics {
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

/// Writes a file into the Cuddle
fn write_file<W: Write + Seek, P: AsRef<Path>>(name: &str, opt_loc: Option<P>, zip: &mut ZipWriter<W>) -> Result<()> {
    if let Some(loc) = opt_loc {
        zip.start_file(format!("{}.{}", name, get_extension(&loc)), FileOptions::default())?;
        let mut f = fs::File::open(loc)?;
        let mut buf = Vec::with_capacity(f.metadata()?.len() as usize);
        f.read_to_end(&mut buf)?;
        zip.write_all(buf.as_slice())?;
    }
    Ok(())
}

/// Retrives the extension of a path
fn get_extension<P: AsRef<Path>>(p: P) -> String {
    if let Some(ext) = p.as_ref().extension() {
        if let Ok(s) = ext.to_os_string().into_string() {
            return s;
        }
    }
    String::new()
}
