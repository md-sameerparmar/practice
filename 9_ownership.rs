fn add_one(e: &mut i32) {
    *e+= 2;
    println!("hello {}", *e);
 }
 fn main() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
 }