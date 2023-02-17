use regex::Regex;

fn main() {
    let pattern = Regex::new(r"[0-9]").unwrap();
    let input = "123";

    if pattern.is_match(input){
        println!("Input is valid!");
    } else {
        println!("Input is not valid!");
    }
    
}