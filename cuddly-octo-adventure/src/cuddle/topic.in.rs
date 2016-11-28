use super::Question;

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub questions: Vec<Question>,
}

impl Default for Topic {
    fn default() -> Topic {
        Topic {
            name: "Hitch Hiker".to_owned(),
            questions: vec![Question::default()],
        }
    }
}
