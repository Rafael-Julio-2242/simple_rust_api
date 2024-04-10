use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateNoteDTO {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>
}


