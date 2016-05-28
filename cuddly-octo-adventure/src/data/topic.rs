use super::Question;

pub struct Topic {
    name: String,
    questions: Vec<Question>,
}

impl Default for Topic {
    fn default() -> Topic {
        Topic {
            name: "Hitch Hiker".to_owned(),
            questions: Vec::new(),
        }
    }
}
