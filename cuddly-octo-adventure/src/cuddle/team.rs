#[derive(Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub name: String,
    pub color: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub score: u32,
}

impl Default for Team {
    fn default() -> Team {
        Team {
            name: "Octocats".to_owned(),
            color: "".to_owned(),
            score: 0,
        }
    }
}
