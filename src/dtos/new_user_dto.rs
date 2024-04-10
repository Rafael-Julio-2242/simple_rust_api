use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserDTO {
    pub name: String,
    pub password: String
}

impl NewUserDTO {
    pub fn new(name: String, password: String) -> NewUserDTO {
        NewUserDTO { name, password }
    }
}
