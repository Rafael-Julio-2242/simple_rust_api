
pub struct User {
    id: i32,
    name: String,
    password: String
}



impl User {
    pub fn new (id: i32, name: String, password: String) -> User {
        User { id, name, password }
    }
    pub fn get_id(&self) -> i32 {
        self.id.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}
