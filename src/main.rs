use regex::Regex;

fn main() {

    let username = "john_doe123";
    let re = Regex::new(r#"^(?!_)(?=.*[!@#$%^&*()])(?=.*[A-Z])(?=.*[0-9])(?=.*[a-z])[A-Za-z0-9!@#$%^&*()]{3,12}$"#).unwrap();
    
    if re.is_match(username) {
        println!("Valid username!");
    } else {
        println!("Invalid username!");
    }
}