use crate::libs::session::Session;
use crate::models;
use crate::models::Question;

use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};

use rand::distributions::{Distribution, Uniform};
use rocket::State;
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
        Ok(())
    }

    pub fn read(state: State<Session>) -> Option<Vec<Question>> {
        let obj_key = "obj";
        if !state
            .session_dic
            .read()
            .expect("No state")
            .contains_key(obj_key)
        {
            let jp = JsonProvider {
                file_name: "src/data/interview.json",
            };

            match jp.parse_file() {
                None => None,
                Some(q) => state
                    .session_dic
                    .write()
                    .expect("No state")
                    .insert(obj_key, q),
            };
        }
        state
            .session_dic
            .read()
            .expect("No state")
            .get(obj_key)
            .and_then(|q| Some(q.to_vec()))
    }

    fn update(question: Question) -> Result<(), ()> {
        Ok(())
    }

    pub fn delete(id: Uuid) -> Result<(), ()> {
        Ok(())
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

    pub fn save_file(&self, questions: &Vec<Question>) -> Result<(), Error> {
        File::create(&self.file_name).and_then(|mut file| {
            let json = serde_json::to_string(&questions).expect("Error");
            file.write_all(json.as_ref())
        })
    }
}

pub fn randomize_questions(state: State<Session>, amount: u64) -> Result<Vec<Question>, String> {
    match QuestionRepo::read(state) {
        None => Err(format!("No question resources")),
        Some(questions) => {
            let uamount = usize::try_from(amount).expect("Error while parsing");
            if questions.len() < uamount {
                Err(format!("Amount is bigger then list of questions"))
            } else if !questions.is_empty() {
                let mut rng = rand::thread_rng();
                let gen_num = Uniform::from(0..questions.len());
                let mut checked_set = HashSet::new();
                let mut q = Vec::new();

                while my_set.len() != uamount {
                    let gen_number = gen_num.sample(&mut rng);
                    if checked_set.insert(gen_number) {
                        q.push(
                            questions
                                .get(usize::try_from(gen_number).expect("Error while parsing"))
                                .expect("Question doesn't exist")
                                .clone(),
                        );
                    }
                }
                Ok(q)
            } else {
                Err(format!("Empty Question"))
            }
        }
    }
}
