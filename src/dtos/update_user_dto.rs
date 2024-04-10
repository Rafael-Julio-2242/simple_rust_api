use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserDTO {
    pub id: i64,
    pub name: Option<String>,
    pub password: Option<String>
}