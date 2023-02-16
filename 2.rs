fn main() {
    let salary = 100.00;
    let salary = 1.50 ; 
    // reads first salary
    println!("The value of salary is :{}",salary);

    let name = "Sam";
    let name_len = name.len();
    println!("Name is {} Length is: {}",name,name_len);

    const word:&str = "abc";
    const uword:usize = word.len();
    println!("{}",uword);

    let company:&'static str = "TutorialsPoint";
   let location:&'static str = "Hyderabad";
   println!("company is : {} location :{}",company,location);
 }