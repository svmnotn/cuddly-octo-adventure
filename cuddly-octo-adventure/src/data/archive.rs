use super::Topic;
use super::settings::Settings;

pub struct Archive {
  info: ArchiveInfo,
  topics: Vec<Topic>,
  settings: Settings,
}

#[derive(Serialize, Deserialize)]
pub struct ArchiveInfo {
  name: String,
  description: String,
  version: String,
  author: String,
  license: String,
  #[serde(rename="updateURL")]
  update_url: String, // TODO Check for URL stuff in std
}
