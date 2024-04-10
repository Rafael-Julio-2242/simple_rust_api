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
}
