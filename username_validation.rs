use std::io;

fn validation_check(username: &str) -> bool {

    let mut is_valid = true;
    if username.chars().next() == Some('_'){
        is_valid = false;
        println!("Username cannot start with an underscore");
    }
    if username.len() < 3 && username.len() > 12 {
        is_valid = false;
        println!("Username must be between 3 to 12 characters long");
    }
    if username.is_lowercase() {
        
    }
    


    is_valid
}

fn main() {
    println!("Enter Username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username);
    let username = &username.trim();
    // println!("string is {}", &res.len());

    if validation_check(username) {
        println!("Valid Username");
    } else {
        println!("Invalid Username");
    }
}
