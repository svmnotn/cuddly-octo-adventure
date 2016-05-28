use super::Topic;
use super::settings::Settings;

pub struct Archive {
  info: ArchiveInfo,
  topics: Vec<Topic>,
  settings: Settings,
}

impl Default for Archive {
  fn default() -> Archive {
    Archive {
      info: ArchiveInfo::default(),
      topics: Vec::new(),
      settings: Settings::default(),
    }
  }
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

impl Default for ArchiveInfo {
  fn default() -> ArchiveInfo {
    ArchiveInfo {
      name: "Default Cuddle".to_owned(),
      description: "This is an auto-generated cuddle whose description has not been changed.".to_owned(),
      version: "1.0.0".to_owned(),
      author: "".to_owned(),
      license: "MIT".to_owned(),
      update_url: "".to_owned(),
    }
  }
}
