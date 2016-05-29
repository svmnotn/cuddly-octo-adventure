#[derive(Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    // color: color
    // font: font
    #[serde(skip_serializing, skip_deserializing)]
    pub score: u32,
}

impl Default for Team {
    fn default() -> Team {
        Team {
            name: "Octocats".to_owned(),
            score: 0,
        }
    }
}
