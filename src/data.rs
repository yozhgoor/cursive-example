use crate::user::User;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProgramData {
    pub user_list: HashMap<String, User>,
    pub active_user: Option<String>,
}

impl ProgramData {
    pub fn mock() -> Self {
        let mut user_list = HashMap::new();
        let blank = String::from("blank");
        let custom = String::from("Custom");

        user_list.insert(blank.clone(), User::new(blank));
        user_list.insert(custom.clone(), User::new(custom));

        Self {
            user_list,
            active_user: None,
        }
    }
}
