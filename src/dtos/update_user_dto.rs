use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserDTO {
    pub id: i64,
    pub name: Option<String>,
    pub password: Option<String>
}

impl UpdateUserDTO {
    pub fn new(id: i64, name: Option<String>, password: Option<String>) -> UpdateUserDTO {
        UpdateUserDTO { id, name, password }
    }
}
