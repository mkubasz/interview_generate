use serde::{Deserialize, Serialize};

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
    id: Option<String>,
    question: String,
    answer: String,
    level: Level,
    tags: Vec<Tag>,
}

struct Filter {
    number: u16,
    level: Level,
    tags: Vec<Tag>,
}
