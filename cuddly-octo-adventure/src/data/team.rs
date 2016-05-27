#[derive(Serialize, Deserialize)]
pub struct Team {
  name: String,
  // color: color
  // font: font
  #[serde(skip_serializing, skip_deserializing)]
  score: u32,
}
