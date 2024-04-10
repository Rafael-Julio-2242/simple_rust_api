use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Note {
    id: i64,
    title: String,
    description: String,
    user_id: i64
}

impl Note {
    pub fn new(id: i64, title: String, description: String, user_id: i64) -> Note {
        Note { id, title, description, user_id }
    }
}

