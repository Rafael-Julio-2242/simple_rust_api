use sqlite::State;

use crate::models::notes::Note;

pub fn get_all_notes() -> Result<Vec<Note>, String> {
    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(con) => {
            let query = "
                SELECT
                    id,
                    title,
                    description,
                    user_id
                FROM note
            ";

            let statement_result = con.prepare(query);

            match statement_result {
                Ok(mut statement) => {
                    let mut notes = Vec::<Note>::new();
                    while let Ok(State::Row) = statement.next() {
                        let id: i64 = statement.read::<i64, _>("id").unwrap();
                        let title: String = statement.read::<String, _>("title").unwrap();
                        let description: String =
                            statement.read::<String, _>("description").unwrap();
                        let user_id: i64 = statement.read::<i64, _>("user_id").unwrap();
                        notes.push(Note::new(id, title, description, user_id));
                    }

                    Ok(notes)
                }
                Err(err) => {
                    return Err(err.to_string());
                }
            }
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn get_user_notes(user_id: i64) -> Result<Vec<Note>, String> {
    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(con) => {
            let query = format!(
                "
                    SELECT
                        id,
                        title,
                        description,
                        user_id
                    FROM note
                    WHERE user_id = {}
                ",
                user_id
            );

            let statement_result = con.prepare(query);

            match statement_result {
                Ok(mut statement) => {
                    let mut notes = Vec::<Note>::new();
                    while let Ok(State::Row) = statement.next() {
                        let id: i64 = statement.read::<i64, _>("id").unwrap();
                        let title: String = statement.read::<String, _>("title").unwrap();
                        let description: String =
                            statement.read::<String, _>("description").unwrap();
                        let user_id: i64 = statement.read::<i64, _>("user_id").unwrap();
                        notes.push(Note::new(id, title, description, user_id));
                    }

                    Ok(notes)
                }
                Err(err) => {
                    return Err(err.to_string());
                }
            }
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn new_note(title: String, description: String, user_id: i64) -> Result<(), String> {

    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(con) => {
            let query = format!(
                "
                    INSERT INTO note
                        (title, description, user_id)
                    VALUES
                        ('{}', '{}', {})
                ",
                title, description, user_id
            );

            let r = con.execute(query);

            if r.is_err() {
                return Err(r.unwrap_err().to_string());
            }
            
            Ok(())

        },
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn delete_note(id: i64) -> Result<(), String> {

    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(con) => {


            let query_validacao = format!(
                "
                    SELECT
                        id,
                        title,
                        description,
                        user_id
                    FROM note
                    WHERE id = {}
                ",
                id
            );

            let statement_result = con.prepare(query_validacao);

            if statement_result.is_err() {
                return Err("An error ocorried!".to_string());
            }

            let mut statement = statement_result.unwrap();
            let mut count = 0;
            while let Ok(State::Row) = statement.next() {
                count += 1;
            }

            if count == 0 {
                return Err("Note not found".to_string());
            }


            let query = format!(
                "
                    DELETE FROM note
                    WHERE id = {}
                ",
                id
            );

            let r = con.execute(query);

            if r.is_err() {
                return Err(r.unwrap_err().to_string());
            }


            Ok(())

        },
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn update_note(id: i64, title: Option<String>, description: Option<String>) -> Result<(), String> {

    if title.is_none() && description.is_none() {
        return Err(String::from("Nothing to change!"));
    }

    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(con) => {


            let mut query = String::from("UPDATE note ");

            if title.is_some() {
                query += format!("SET title = '{}' ", title.unwrap()).as_str();
            }
            if description.is_some() {
                if query.contains("SET") {
                    query += format!(", description = '{}' ", description.unwrap()).as_str();
                } else {
                    query += format!("SET description = '{}' ", description.unwrap()).as_str();
                }
            }

            query += format!("WHERE id = {} ", id).as_str();

            let r = con.execute(query);

            if r.is_err() {
                return Err(r.unwrap_err().to_string());
            }

            Ok(())
        },
        Err(err) => {
            return Err(err.to_string());
        }
    }

}


