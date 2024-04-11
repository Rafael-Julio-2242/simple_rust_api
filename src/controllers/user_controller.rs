use sqlite::State;

use crate::models::user::User;

pub fn get_all_users() -> Result<Vec<User>, String> {
    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(connection) => {
            let query = "
            SELECT 
                id,
                name,
                password
            FROM user 
        ";

            let statement_result = connection.prepare(query);

            match statement_result {
                Ok(mut statement) => {
                    let mut users = Vec::<User>::new();

                    while let Ok(State::Row) = statement.next() {
                        let id: i64 = statement.read::<i64, _>("id").unwrap();
                        let name: String = statement.read::<String, _>("name").unwrap();
                        let password: String = statement.read::<String, _>("password").unwrap();
                        users.push(User::new(id, name, password));
                    }

                    Ok(users)
                }
                Err(error) => {
                    return Err(error.to_string());
                }
            }
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}

pub fn get_user(id: i64) -> Result<User, String> {
    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(connection) => {
                let query = format!("
                        SELECT 
                            id,
                            name,
                            password
                        FROM user
                        WHERE id = {}
                    ", id);
                
                let statement_result = connection.prepare(query);

                match statement_result {
                    Ok(mut statement) => {

                        statement.next().unwrap(); // To make things Work !

                        let id  = statement.read::<i64, _>(0).unwrap();
                        let name  = statement.read::<String, _>(1).unwrap();
                        let password  = statement.read::<String, _>(2).unwrap();

                        Ok(User::new(id, name, password))
                    },
                    Err(error) => {
                        return Err(error.to_string());
                    }
                }


        },
        Err(error) => {
            return Err(error.to_string());
        }
    }

}

pub fn new_user(name: String, password: String) -> Result<(), String> {

    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(connection) => {
            let query = format!("
                        INSERT INTO user (name, password)
                        VALUES ('{}', '{}')
                    ", name, password);
            let r = connection.execute(query);

            if r.is_err() {
                return Err(r.unwrap_err().to_string());
            }

            Ok(())
        },
        Err(error) => {
            return Err(error.to_string());
        }
    }

}

pub fn delete_user(id: i64) -> Result<(), String> {

    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(connection) => {

            // Preciso validar se o usuário existe!
            let query_validacao = format!("
                SELECT 
                    id,
                    name,
                    password
                FROM user
                WHERE id = {}
            ", id);

            let statement_result = connection.prepare(query_validacao);

            if statement_result.is_err() {
                return Err("An error ocorried!".to_string());
            }

            let mut statement = statement_result.unwrap();
            let mut count = 0;
            while let Ok(State::Row) = statement.next() {
                count += 1;
            }

            if count == 0 {
                return Err("User not found".to_string());
            }

            let query = format!("
                        DELETE FROM user
                        WHERE id = {}
                    ", id);
            let r = connection.execute(query);

            if r.is_err() {
                return Err(r.unwrap_err().to_string());
            }

            Ok(())
        },
        Err(error) => {
            return Err(error.to_string());
        }
    }

}

pub fn update_user(id: i64, name: Option<String>, password: Option<String>) -> Result<(), String> {

    if name.is_none() && password.is_none() {
        return Err(String::from("Nothing to change!"));
    }

    let con_result = sqlite::open("sqlite.db");

    match con_result {
        Ok(connection) => {
            
            // Contruir a query de acordo!

            let mut query = String::from("UPDATE user ");

            if name.is_some() {
                query += format!("SET name = '{}' ", name.unwrap()).as_str();
            }

            if password.is_some() {
                if query.contains("SET") {
                    query += format!(", password = '{}' ", password.unwrap()).as_str();
                } else {
                    query += format!("SET password = '{}' ", password.unwrap()).as_str();
                }
            }

            query += format!("WHERE id = {}", id).as_str();

            let r = connection.execute(query);

            if r.is_err() {
                let error_string = r.unwrap_err().to_string();
                println!("Error: {}", error_string.clone());
                return Err(error_string);
            }

            Ok(())

        },
        Err(error) => {
            return Err(error.to_string());
        }
    }
    
}
