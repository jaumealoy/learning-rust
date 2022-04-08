use std::collections::HashMap;
use std::vec;

use chrono::{Utc, DateTime, NaiveDateTime};
use mysql_async::prelude::{Queryable, ToValue};
use mysql_async::{Row, Params, Value};

use crate::models::{User, UserCollection};
use crate::database;

pub async fn get_users() -> UserCollection {
    let mut con = database::get_connection().await.unwrap();
    let mut result = UserCollection::new();

    let rows = con.query::<Row, &str>("SELECT id, email, name, age, created FROM users")
        .await
        .unwrap();

    for row in rows {
        let date: String = row.get(4).unwrap();
        let naive_date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let utc_time = DateTime::<Utc>::from_utc(naive_date, Utc);

        result.push(User {
            id: row.get(0).unwrap(),
            email: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            age: row.get(3).unwrap(),
            created: utc_time
        });

    }

    result
}

pub async fn create_user<'a>(user: &'a mut User) -> &'a mut User {
    let mut con = database::get_connection().await.unwrap();
    
    let params = Params::Positional(vec![
        user.email.to_value(),
        user.name.to_value(),
        user.age.to_value(),
        user.created.format("%Y-%m-%d %H:%M:%S").to_string().to_value()
    ]);

    let insert = con
        .exec_drop("INSERT INTO users VALUES (NULL, ?, ?, ?, ?)", params)
        .await;
    
    let last_id = con.last_insert_id().unwrap();

    user.id = last_id as usize;
    user
}

pub async fn get_user(id: usize) -> Option<User> {
    let mut connection = database::get_connection().await.unwrap();

    let params = Params::Positional(vec![id.to_value()]);
    let row = connection
        .exec_first::<Row, &str, Params>("SELECT name, email, age, created FROM users WHERE id = ?", params)
        .await
        .unwrap();

    if let Some(result) = row {
        Some(
            User {
                id: id,
                email: result.get("email").unwrap(),
                age: result.get("age").unwrap(),
                name: result.get("name").unwrap(),
                created: Utc::now()
            }
        )
    } else {
        None
    }
}

pub async fn save_user(user: &User) {
    let mut connection = database::get_connection().await.unwrap();
    
    let params = Params::Named(
        HashMap::from([
            ("email".to_owned(), user.email.to_value()),
            ("name".to_owned(), user.name.to_value()),
            ("age".to_owned(), user.age.to_value()),
            ("id".to_owned(), user.id.to_value())
        ])
    );

    connection
        .exec_drop("UPDATE users SET email = :email, name = :name, age = :age WHERE id = :id", params)
        .await;
}

pub async fn delete_user(user_id: usize) -> bool {
    let mut connection = database::get_connection().await.unwrap();

    let params = Params::Positional(vec![user_id.to_value()]);

    connection
        .exec_drop("DELETE FROM users WHERE id = ? LIMIT 1", params)
        .await;

    connection.affected_rows() > 0
}