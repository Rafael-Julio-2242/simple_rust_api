use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewNoteDTO {
    pub title: String,
    pub description: String,
    pub user_id: i64
}


