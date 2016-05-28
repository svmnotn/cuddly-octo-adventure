use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Answer {
    correct: bool,
    #[serde(rename="answer")]
    text: String,
    // #[serde(skip_serializing, skip_deserializing)]
    // img: Option<Image>, from gtk
    #[serde(rename="imageLoc", skip_serializing_if="Option::is_none", default)]
    img_loc: Option<PathBuf>,
}

impl Default for Answer {
    fn default() -> Answer {
        Answer {
            correct: false,
            text: "42".to_owned(),
            img_loc: None,
        }
    }
}
