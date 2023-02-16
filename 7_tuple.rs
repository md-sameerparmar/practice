fn main(){
    let tuple: (i32,f64,u8) = (1,2.6,3);
    println!("{:?}",tuple.0);
    println!("{:?}",tuple.1);
    println!("{:?}",tuple.2);
    my_func(tuple);

    let b:(i32,bool,f64) = (30,true,7.9);
   print(b);
}fn my_func(a:(i32,f64,u8)){
    println!("{:?}",a)
}
fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    let (age,is_male,cgpa) = x; //assigns a tuple to 
    println!("Age is {} , isMale? {},cgpa is 
    {}",age,is_male,cgpa);
 }