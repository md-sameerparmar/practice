use regex::Regex;
extern crate regex;

fn validate_username(username: &str) -> bool {
    let re = Regex::new(r"^(?!_)(?=.*[!@#$%^&*()])(?=.*[A-Z])(?=.*[0-9])(?=.*[a-z])[A-Za-z0-9!@#$%^&*()]{3,12}$").unwrap();
    re.is_match(username)
}

fn main() {
    let username1 = "hello_world";
    let username2 = "123";
    let username3 = "user.name";
    
    println!("{} is valid: {}", username1, validate_username(username1));
    println!("{} is valid: {}", username2, validate_username(username2));
    println!("{} is valid: {}", username3, validate_username(username3));
}