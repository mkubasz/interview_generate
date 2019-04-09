use uuid::Uuid;

#[derive(Display)]
pub enum Tag {
    OOP,
}

#[derive(Display)]
enum Level {
    JUNIOR,
    MEDIUM,
    SENIOR,
}

pub struct Question {
    id: Option<Uuid>,
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
