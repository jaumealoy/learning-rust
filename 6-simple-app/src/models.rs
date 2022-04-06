use std::fmt::Display;

use chrono::Utc;

pub struct User {
    pub id: usize,
    pub email: String,
    pub name: String,
    pub age: u8,
    pub created: chrono::DateTime<Utc>
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = format!("#{}\t{}\t{}\t{}", self.id, self.email, self.name, self.created.format("%Y-%m-%d %H:%M"));
        f.write_str(&tmp)
    }
}

pub type UserCollection = Vec<User>;