use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Answer {
  correct: bool,
  #[serde(rename="answer")]
  text: String,
  #[serde(rename="imageLoc")]
  img_loc: PathBuf,
  //#[serde(skip_serializing_if="Option::is_none")]
  //img: Option<Image> from gtk
}
