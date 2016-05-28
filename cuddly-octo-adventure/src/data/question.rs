use std::path::PathBuf;
use super::Answer;

#[derive(Serialize, Deserialize)]
pub struct Question {
  //id: String,
  value: u32,
  #[serde(rename="question")]
  text: String,
  #[serde(rename="imageLoc", skip_serializing_if="Option::is_none", default)]
  img_loc: Option<PathBuf>,
  //#[serde(skip_serializing, skip_deserializing)]
  //img: Option<Image> from gtk
  answers: Vec<Answer>,
}
