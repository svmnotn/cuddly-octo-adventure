use std::path::PathBuf;
use super::Answer;

#[derive(Serialize, Deserialize)]
pub struct Question {
    // id: String,
    pub value: u32,
    #[serde(rename="question")]
    pub text: String,
    #[serde(rename="imageLoc", skip_serializing_if="Option::is_none")]
    pub img_loc: Option<PathBuf>,
    // #[serde(skip_serializing, skip_deserializing)]
    // img: Option<Image> from gtk
    pub answers: Vec<Answer>,
}

impl Default for Question {
    fn default() -> Question {
        Question {
            value: 0,
            text: "What is the answer to life the universe and everything?".to_owned(),
            img_loc: None,
            answers: vec![Answer::default()],
        }
    }
}
