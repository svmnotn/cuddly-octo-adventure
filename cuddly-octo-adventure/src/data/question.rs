use super::Answer;
use gtk::Image;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub value: u32,
    #[serde(rename="question")]
    pub text: String,
    #[serde(rename="imageLoc", skip_serializing_if="Option::is_none")]
    pub img_loc: Option<PathBuf>,
    #[serde(skip_serializing, skip_deserializing)]
    pub img: Option<Image>,
    pub answers: Vec<Answer>,
}

impl Default for Question {
    fn default() -> Question {
        Question {
            id: "0".to_owned(),
            value: 0,
            text: "What is the answer to life the universe and everything?".to_owned(),
            img: None,
            img_loc: None,
            answers: vec![Answer::default()],
        }
    }
}
