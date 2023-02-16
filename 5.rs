fn main(){
    // let nick_name = "Sam";
    // let name = match nick_name{
    //     "Sandy" => "Sanjdeep",
    //     "mack" => "Mayank",
    //     "Sam" => "Sameer",
    //     _ => "Not Match"
    // };
    // println!("Full name is {}",name);

    // for x in 1..11{
    //     if x == 5{
    //         println!("We reached to 5");
    //     }
    //     println!("value of x is {}",x)
    // }

    // let mut x = 0;
    // while x < 10{
    //     x+=1;
    //     println!("inside loop x value is {}",x);
    // }
    // println!("outside loop x value is {}",x);

//     let mut x = 0;
//    loop {
//       x+=1;
//       println!("x={}",x);

//       if x==5 {
//          break;
//       }
//    }

let mut count = 0;

for num in 0..21 {
   if num % 2==0 {
      continue;
   }
   count+=1;
   println!("{}",num)
}
println! (" The count of odd values between 0 and 20 is: {} ",count);
}