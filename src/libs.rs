use crate::libs::session::Session;
use crate::models;
use crate::models::Question;
use rocket::State;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

pub mod session {
    use crate::models::Question;
    use core::borrow::Borrow;
    use std::collections::HashMap;
    use std::sync::RwLock;

    pub struct Session {
        pub session_dic: RwLock<HashMap<&'static str, Vec<Question>>>,
    }

    impl Session {
        pub fn new() -> Self {
            Session {
                session_dic: RwLock::new(HashMap::new()),
            }
        }
    }
}

pub struct QuestionRepo;

impl QuestionRepo {
    fn create(question: Question) -> Result<(), ()> {
        JsonProvider::add_item(question)
    }

    pub fn read(state: State<Session>) -> Option<Vec<Question>> {
        let KEY = "obj";
        if !state
            .session_dic
            .read()
            .expect("No state")
            .contains_key(KEY)
        {
            let jp = JsonProvider {
                file_name: "src/data/interview.json",
            };

            match jp.parse_file() {
                None => None,
                Some(q) => state.session_dic.write().expect("No state").insert(KEY, q),
            };
        }
        state
            .session_dic
            .read()
            .expect("No state")
            .get(KEY)
            .and_then(|q| Some(q.to_vec()))
    }

    fn update(question: Question) -> Result<(), ()> {
        JsonProvider::update_item()
    }

    pub fn delete(id: Uuid) -> Result<(), ()> {
        JsonProvider::delete_item()
    }
}

struct JsonProvider {
    file_name: &'static str,
}

impl JsonProvider {
    pub fn parse_file(&self) -> Option<Vec<Question>> {
        File::open(&self.file_name)
            .and_then(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents).ok();
                let parsed: models::File = serde_json::from_str(&contents).expect("Error");
                Ok(parsed.questions)
            })
            .ok()
    }

    fn add_item(question: Question) -> Result<(), ()> {
        Ok(())
    }

    fn update_item() -> Result<(), ()> {
        Ok(())
    }

    fn delete_item() -> Result<(), ()> {
        Ok(())
    }
}
