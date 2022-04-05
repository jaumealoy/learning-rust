use chrono::Utc;

pub struct User {
    pub id: usize,
    pub email: String,
    pub name: String,
    pub age: u8,
    pub created: chrono::DateTime<Utc>
}

pub type UserCollection = Vec<User>;