#[derive(Serialize, Deserialize)]
pub struct Team {
    name: String,
    // color: color
    // font: font
    #[serde(skip_serializing, skip_deserializing)]
    score: u32,
}

impl Default for Team {
    fn default() -> Team {
        Team {
            name: "Octocats".to_owned(),
            score: 0,
        }
    }
}
