use serde::{Deserialize, Serialize};

#[derive(Display, Serialize, Deserialize)]
pub enum Tag {
    OOP,
}

#[derive(Display, Serialize, Deserialize)]
enum Level {
    JUNIOR,
    MEDIUM,
    SENIOR,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize)]
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
