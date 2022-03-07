#[derive(Debug)]
pub struct ProgramData {
    pub user_list: Vec<String>,
    pub active_user: Option<String>
}

impl ProgramData {
    pub fn mock() -> Self {
        Self {
            user_list: vec!["blank", "Custom"],
            active_user: None,
        }
    }
}
