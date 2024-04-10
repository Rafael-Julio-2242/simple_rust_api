use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserDTO {
    pub name: String,
    pub password: String
}
