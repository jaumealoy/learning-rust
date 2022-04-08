mod models;
mod database;
mod user_dao;
use std::io::Write;

use chrono::DateTime;
use models::{UserCollection, User};
use mysql_async::prelude::Queryable;

#[tokio::main]
async fn main() {
    // load .env fil
    dotenv::dotenv();

    // init database
    database::init().await;

    loop {
        let selected_option = show_menu();
        println!("-> {}", selected_option);

        if selected_option == 0 {
            let users: UserCollection = user_dao::get_users().await;

            for user in users {
                println!("{}", user);
            }
        } else if selected_option == 1 {
            let mut new_user = User {
                id: 0,
                name: text_input("Name: "),
                email: text_input("Email: "),
                age: 37,
                created: chrono::Utc::now()
            };

            user_dao::create_user(&mut new_user).await;

            println!("User inserted with id {}", new_user.id);
        } else if selected_option == 2 {
            println!("Insert user id: ");
            let user_id = integer_input();
            if user_dao::delete_user(user_id).await {
                println!("User has been deleted");
            } else {
                println!("Couldn't delete user.");
            }
        } else if selected_option == 3 {
            println!("Insert user id: ");
            let user_id = integer_input();

            if let Some(mut user) = user_dao::get_user(user_id).await {
                println!("{}", user);

                println!("Insert the new values or leave them blank.");
                let name = text_input("Name: ");
                if name.len() > 0 {
                    user.name = name;
                }

                let email = text_input("Email: ");
                if email.len() > 0 {
                    user.email = email;
                }

                let age = text_input("Age: ");
                if age.len() > 0 {
                    user.age = age.parse().unwrap();
                }

                user_dao::save_user(&user)
                    .await;

                println!("Data has been saved.");
            } else {
                println!("User does not exist.");
            }
        } else if selected_option == 4 {
            println!("Insert user id: ");
            let user_id = integer_input();
            
            if let Some(user) = user_dao::get_user(user_id).await {
                println!("{}", user);
            } else {
                println!("User does not exist.");
            }
        } else if selected_option == 5 {
            println!("Good bye!");
            break;
        }
    }

    database::close().await;
}

fn show_menu() -> u8 {
    let menu_options = vec!(
        "List users",
        "Create a new user",
        "Delete an user",
        "Edit an user",
        "Show an user",
        "Exit"
    );

    let mut i = 0;
    for option in &menu_options {
        println!("{}) {}", i, option);
        i += 1;
    }

    let mut valid_option = false;
    let mut option: u8 = menu_options.len() as u8;
    while !valid_option {
        let mut buffer = String::new();
        let input = std::io::stdin();

        let mut eol = 0;
        if let Ok(_) = input.read_line(&mut buffer) {
            for char in buffer.chars() {
                if !(char == '\n' || char == '\r') {
                    eol += 1;
                }
            }
        }

        option = menu_options.len() as u8;
        let conversion = buffer[0..eol].parse::<u8>();

        if let Ok(result) = conversion {
            option = result;
            valid_option = true;
        }

        if option >= menu_options.len() as u8 {
            println!("Invalid option. Insert a value between 0 and {}", menu_options.len() - 1);
        }
    }

    option
}

fn text_input(prompt: &str) -> String {
    let mut buffer = String::new();

    print!("{}", prompt);
    std::io::stdout().flush();

    let input = std::io::stdin();
    input.read_line(&mut buffer);

    let mut line_end = 0;
    for character in buffer.chars() {
        if character == '\n' {
            break;
        }

        line_end += 1;
    }

    buffer[0..line_end - 1].to_owned()
}

fn integer_input() -> usize {
    let mut buffer = String::new();
    


    let mut value: Option<usize> = None;
    while let None = value {
        let input = std::io::stdin();
        input.read_line(&mut buffer);
    
        let mut line_end = 0;
        for character in buffer.chars() {
            if character == '\n' {
                break;
            }
            line_end += 1;

        }

        let result = buffer[0..line_end - 1].parse::<usize>();
        if let Ok(x) = result {
            value = Some(x);
        } else {
            println!("Invalid number. Insert a valid number.");
        }
    }

    value.unwrap()
}