use super::Topic;
use super::settings::Settings;

#[derive(Debug)]
pub struct Archive {
    pub info: ArchiveInfo,
    pub topics: Vec<Topic>,
    pub settings: Settings,
}

impl Default for Archive {
    fn default() -> Archive {
        Archive {
            info: ArchiveInfo::default(),
            topics: vec![Topic::default()],
            settings: Settings::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchiveInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub license: String,
    #[serde(rename="updateURL")]
    pub update_url: String, // TODO Check for URL stuff in std
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
