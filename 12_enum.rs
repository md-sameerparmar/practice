// #[derive(Debug)]
// enum Gender{
//     Male, Female
// }

// #[derive(Debug)]
// struct Person{
//     name: String,
//     gender: Gender
// }

// fn main() {
//     let p1 = Person{name:String::from("Sameer"), gender: Gender::Male};
//     let p2 = Person{name:String::from("Grusha"), gender: Gender::Female};

//     // println!("Name is {} Gender is {:?}", p1.name, p1.Gender);
//     // println!("Name is {} Gender is {:?}", p2.name, p2.Gender);

//     println!("{:?}", p1);
//     println!("{:?}", p2);

// }

// enum option----------------------------------------------------------

fn main(){
    let res = is_even(1);
    println!("{:?}", res);
    println!("{:?}", is_even(2));
    
}

fn is_even(num: i32) -> Option<bool> {
    if num % 2 == 0{
        Some(true)
    }
    else{
        None
    }
}