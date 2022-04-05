mod models;
use models::{UserCollection, User};

#[tokio::main]
async fn main() {
    loop {
        let selected_option = show_menu();
        println!("-> {}", selected_option);

        if selected_option == 0 {
            let users: UserCollection = vec![];

            for user in users {
                print_user(&user);
            }
        } else if selected_option == 4 {
            println!("Good bye!");
            break;
        }
    }
}

fn show_menu() -> u8 {
    let menu_options = vec!(
        "List users",
        "Create a new user",
        "Delete an user",
        "Edit an user",
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

fn print_user(user: &User) {
    println!("#{}\t{}\t{}\t{}", user.id, user.email, user.name, user.created.format("%Y-%m-%d"));
}