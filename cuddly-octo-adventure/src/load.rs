use ::cuddle::*;
use ::error::Result;
use json::de as from_json;
use std::fs;
use std::io::Read;
use std::path::Path;
use tempdir::TempDir;
use zip::{CompressionMethod, ZipArchive};
use zip::read::ZipFile;

/// Checks if the path contains the given file/folder
fn contains<P: AsRef<Path>>(path: P, name: &str) -> bool {
    if let Ok(entries) = fs::read_dir(path) {
        for rentry in entries {
            if let Ok(entry) = rentry {
                if let Ok(n) = entry.file_name().into_string() {
                    if n == name {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Decompress the ZipFile into the folder
fn write_file<P: AsRef<Path>>(folder: P, file: ZipFile) -> Result<()> { Ok(()) }

/// Load a 'cuddle' from disk, a 'cuddle' is a ziped folder.
/// Any extra files will be extracted into the temporary folder returned,
/// all paths are modified to lead to that folder.
pub fn load_cuddle<P: AsRef<Path>>(from: P, folder: Option<P>) -> Result<(Archive, TempDir)> {
    let mut archive = Archive::default();

    let f = fs::File::open(from)?;
    let mut cuddle = ZipArchive::new(f)?;

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
                            // We got a picture! So, lets make sure this topic has a folder

                            if contains(dir.path(), n) == false {
                                fs::create_dir(dir.path().join(n))?;
                            }

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
