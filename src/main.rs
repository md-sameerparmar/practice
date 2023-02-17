// use regex::Regex;

// fn main() {

//     let username = "john_doe123";
//     let re = Regex::new(r#"^(?!_)(?=.*[!@#$%^&*()])(?=.*[A-Z])(?=.*[0-9])(?=.*[a-z])[A-Za-z0-9!@#$%^&*()]{3,12}$"#).unwrap();
    
//     if re.is_match(username) {
//         println!("Valid username!");
//     } else {
//         println!("Invalid username!");
//     }
// }
use regex::Regex;

fn main(){
    let text = "user.logins,host=webserver01,country=US:1|c";

    let re = Regex::new(r"([^,]+)((,[a-zA-Z0-9=]+)*)(:[0-9]+\|[c])").expect("failed to compile regex ");
    for cap in re.captures_iter(text){
        println!("Metric: {}",&cap[1]);
        println!("Tags: {}",&cap[2]);
        println!("Tags: {}",&cap[3]);
    } 

}