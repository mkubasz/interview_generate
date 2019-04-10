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
