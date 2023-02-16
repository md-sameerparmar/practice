fn main(){
    let mut emt_str = String::new();
    println!("{}",emt_str.len());
    emt_str.push_str("hey");
    println!("{}",emt_str);

    let fill_str = String::from("Sam");
    println!("{}",fill_str.len());

    // to_string
    let num = 12.to_string();
    let word1 = "Minddeft";
    let res = num+word1;
    println!("{}",res);

    // replace
    let word2 = "hey there there there!";
    let word3 = word2.replace("there", "everyone");
    println!("{}",word3);

    // as str
    let word4 = String::from("example string");
    print_literal(word4.as_str());

    // push
    let mut company = "Tutorial".to_string();
   company.push('s'); //for character
   company.push_str(" Point"); //for string
   println!("{}",company);

   //trim
   let fullname = " Tutorials Point \r\n";
   println!("{}",fullname);
   println!("Before trim ");
   println!("length is {}",fullname.len());
   println!();
   println!("After trim ");
   println!("length is {}",fullname.trim().len());

   

}

fn print_literal(data:&str){
    println!("{}",data);
}