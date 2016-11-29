use ::cuddle::*;
use ::error::Result;
use json::de as from_json;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use zip::{CompressionMethod, ZipArchive};
use zip::read::ZipFile;

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

    archive.topics = Vec::<Topic>::new();

    // Iterate over every other file
    for i in 0..cuddle.len() {
        let mut f = cuddle.by_index(i)?;
        println!("{}:{} - {:?}", i, f.name(), f.compression());
        let name = String::from(f.name());

        if name.contains("bg_img") {
            // This is the bg image, decompress it to the folder
            let path = write_file(dir.path(), f)?;
            // Update the path in the Archive
            archive.settings.bg_loc = Some(path);
        } else if name.contains("bg_sound") {
            // This is the bg sound, decompress it to the folder
            let path = write_file(dir.path(), f)?;
            // Update the path in the Archive
            archive.settings.sound.bg_sound_loc = Some(path);
        } else if name.contains("yay_sound") {
            // This is the yay sound, decompress it to the folder
            let path = write_file(dir.path(), f)?;
            // Update the path in the Archive
            archive.settings.sound.yay_sound_loc = Some(path);
        } else if name.contains("nay_sound") {
            // This is the nay sound, decompress it to the folder
            let path = write_file(dir.path(), f)?;
            // Update the path in the Archive
            archive.settings.sound.nay_sound_loc = Some(path);
        } else if let CompressionMethod::Deflated = f.compression() {
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

                    if let Some(t) = archive.topics.iter_mut().find(|t| t.name == n) {
                        if let Some(q) = t.questions.iter_mut().find(|q| q.id == id) {
                            if id.contains("ans") {
                                // It is an answer, so lets get it's number
                                if let Some(num) = id.split('-').nth(2) {
                                    // Lets also make sure it is a valid number
                                    // (It should be if save_cuddle was used)
                                    if let Ok(i) = num.parse() {
                                        // Now lets find the answer that maches it
                                        if let Some(a) = q.answers.iter_mut().nth(i) {
                                            // if let Some(a) = q.answers.get_mut(i) {
                                            // ^ leads to the issue in line 104
                                            // Now this is the correct answer, so lets decompress the picture
                                            let path = write_file(dir.path().join(n), f)?;
                                            // Update the path in the Archive
                                            a.img_loc = Some(path);
                                            // ^^^^^^^ this causes issues with
                                            //  rustc 1.15.0-nightly (03bdaade2 2016-11-27)
                                        }
                                    }
                                }
                            } else {
                                // It is a question, so let decompress the picture
                                let path = write_file(dir.path().join(n), f)?;
                                // Update the path in the Archive
                                q.img_loc = Some(path);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok((archive, dir))
}

/// Decompress the ziped file into the folder
fn write_file<P: AsRef<Path>>(folder: P, mut file: ZipFile) -> Result<PathBuf> {
    let path = PathBuf::from(folder.as_ref().join(file.name()));
    let mut f = fs::File::create(&path)?;
    let mut buf = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut buf)?;
    f.write_all(buf.as_slice())?;
    Ok(path)
}

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
