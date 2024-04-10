use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: i64,
    name: String,
    password: String
}

impl User {
    pub fn new(id: i64, name: String, password: String) -> User {
        User { id, name, password }
    }
    pub fn get_id(&self) -> i64 {
        self.id.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}
