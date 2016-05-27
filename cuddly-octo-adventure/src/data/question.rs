use std::path::PathBuf;
use super::Answer;

#[derive(Serialize, Deserialize)]
pub struct Question {
  id: String,
  value: u32,
  #[serde(rename="question")]
  text: String,
  #[serde(rename="imageLoc")]
  img_loc: PathBuf,
  //#[serde(skip_serializing_if="Option::is_none")]
  //img: Option<Image> from gtk
  answers: Vec<Answer>,
}
