fn main(){
    let float_separator = 11_000;
    println!("float value {}", float_separator);

    let mood:bool = true;
    let emj:char = '🥵';
    println!("Yesterday was a hectic day right?{} {}",emj,mood);
    let fees = 25_000;
   let salary:f64 = 35_000.00;
   println!("fees is {} and salary is {}",fees,salary);

   let mut fees = 25000;
   println!("fees is : {}", fees);
   fees = 35000;
   println!("fees is : {}", fees);

   const NAME:&str = "Sam";
   const VAL:i8 = 20;
   println!("Name is: {}", NAME);
   println!("Val is: {}", VAL);
}