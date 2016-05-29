use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub correct: bool,
    #[serde(rename="answer")]
    pub text: String,
    // #[serde(skip_serializing, skip_deserializing)]
    // img: Option<Image>,
    #[serde(rename="imageLoc", skip_serializing_if="Option::is_none")]
    pub img_loc: Option<PathBuf>,
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
