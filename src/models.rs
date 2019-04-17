use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Display, Serialize, Deserialize, Clone)]
pub enum Tag {
    OOP,
    JUNIOR,
    MEDIUM,
    SENIOR,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: Option<String>,
    pub question: String,
    pub answer: String,
    pub tags: Vec<Tag>,
}

impl Hash for Question {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.question.hash(state);
        self.answer.hash(state);
    }
}

struct Filter {
    number: u16,
    tags: Vec<Tag>,
}
