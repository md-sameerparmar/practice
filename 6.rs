fn main(){
    // let mut a = 5;
    // fun(&mut a);
    // println!("The value of na is:{}",a);

    let name:&str = "Sameer";
   display(name.to_string()); 
}
// fn fun(no:&mut i32){
//     *no = 0;
//     // println!("Hello {}",no)
// } 
fn display(param_name:String){
    println!("param_name value is :{}",param_name);
 }