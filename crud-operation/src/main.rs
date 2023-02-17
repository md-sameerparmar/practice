use std::io;

fn validate_username(username: &str) -> bool {
    let mut is_valid = true;
    
    // Check if the username starts with an underscore
    if username.chars().next() == Some('_') {
        is_valid = false;
        println!("\n- Username must not start with an underscore");
    }
    
    // check username must be between 3 to 12 characters long
    if username.len() < 3 || username.len() > 12{
        is_valid = false;
        println!("\n- Username must be between 3 to 12 characters long");
    }
    // Check if the username has at least one special character
    if !username.chars().any(|c| "!@#$%^&*()_".contains(c)) {
        is_valid = false;
        println!("\n- Username must contain at least one of ' !@#$%^&*()_ ' special character");
    }
    // Check if the username has at least one uppercase character
    if !username.chars().any(|c| c.is_ascii_uppercase()) {
        is_valid = false;
        println!("\n- Username must contain at least one uppercase character");

    }
    // Check if the username has at least one lowercase character
    if !username.chars().any(|c| c.is_ascii_lowercase()) {
        is_valid = false;
        println!("\n- Username must contain at least one lowercase character");
    }
    is_valid
}

fn main() {
    println!("Enter a username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    
    if validate_username(username) {
        println!("\nThis username is valid.");
    } else {
        println!("\nThis username is not valid.");
    }
}