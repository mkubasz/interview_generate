use crate::libs::session::Session;
use crate::models;
use crate::models::{Question, Tag};

use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};

use core::borrow::Borrow;
use rand::distributions::{Distribution, Uniform};
use rocket::State;
use uuid::Uuid;

pub mod session {
    use crate::libs::JsonProvider;
    use crate::models::Question;
    use core::borrow::Borrow;
    use rocket::State;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::io::Error;
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
        pub fn get(state: &State<Session>) -> Option<Vec<Question>> {
            let obj_key = "obj";
            state
                .session_dic
                .read()
                .expect("No state")
                .get(obj_key)
                .and_then(|q| Some(q.to_vec()))
        }

        pub fn add(state: &State<Session>, question: &Question) -> Option<Vec<Question>> {
            let obj_key = "obj";
            let mut new_state = state
                .session_dic
                .read()
                .expect("No state")
                .get(obj_key)
                .expect("Test")
                .to_vec();
            new_state.push(question.clone());
            state
                .session_dic
                .write()
                .expect("No state")
                .insert(obj_key, new_state)
        }

        pub fn commit(questions: Vec<Question>) -> Result<(), Error> {
            JsonProvider {
                file_name: "src/data/interview.json",
            }
            .save_file(&questions)
        }
    }
}

pub struct QuestionRepo;

impl QuestionRepo {
    pub fn create(state: State<Session>, question: Question) -> Result<(), Error> {
        Session::commit(Session::add(&state, &question).unwrap())
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
        Session::get(&state)
    }

    pub fn update(state: State<Session>, question: Question) -> Result<(), ()> {
        let mut questions = Session::get(&state).unwrap();
        questions.retain(|p| p.id != question.id);
        questions.push(question.clone());
        Session::commit(questions);
        Session::add(&state, &question);
        Ok(())
    }

    pub fn delete(state: State<Session>, id: Uuid) -> Result<(), ()> {
        let mut questions = Session::get(&state).unwrap();
        questions.retain(|p| p.id != Some(id.to_string()));
        Session::commit(questions);
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
            let file_json = crate::models::File {
                questions: questions.clone(),
            };
            let json = serde_json::to_string(&file_json).expect("Error");
            file.write_all(json.as_ref())
        })
    }
}

pub fn contains(mut tag1: Vec<Tag>, tag2: Vec<Tag>) -> bool {
    let mut contain = false;
    tag1.iter_mut().for_each(|el| {
        if tag2.contains(el) {
            contain = true;
        }
    });
    contain
}

pub fn randomize_questions(
    state: State<Session>,
    amount: u64,
    tags: Vec<Tag>,
) -> Result<Vec<Question>, String> {
    match QuestionRepo::read(state) {
        None => Err(format!("No question resources")),
        Some(mut questions) => {
            let uamount = usize::try_from(amount).expect("Error while parsing");
            //  questions.retain(|p| contains(p.tags.clone(), tags.clone()));
            if questions.len() < uamount {
                Err(format!("Amount is bigger then list of questions"))
            } else if !questions.is_empty() {
                let mut rng = rand::thread_rng();
                let gen_num = Uniform::from(0..questions.len());
                let mut checked_set = HashSet::new();
                let mut q = Vec::new();

                while checked_set.len() != uamount {
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
