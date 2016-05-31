use super::Answer;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub id: String,
    pub value: u32,
    #[serde(rename="question")]
    pub text: String,
    #[serde(rename="imageLoc", skip_serializing_if="Option::is_none")]
    pub img_loc: Option<PathBuf>,
    pub answers: Vec<Answer>,
}

impl Default for Question {
    fn default() -> Question {
        Question {
            id: "0".to_owned(),
            value: 0,
            text: "What is the answer to life the universe and everything?".to_owned(),
            img_loc: None,
            answers: vec![Answer::default()],
        }
    }
}
