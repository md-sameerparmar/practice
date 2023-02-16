fn main(){
    let msg = "Tutorials Point has good t
   utorials".to_string();
   let mut i = 1;

   for word in msg.split_whitespace(){
    println!("word {} {}",i,word);
    i+=1;
   }

   let fullname = "Kannan,Sudhakaran,Tutorialspoint";

   for token in fullname.split(","){
      println!("token is {}",token);
   }

   //store in a Vector
   println!("\n");
   let tokens:Vec<&str>= fullname.split(",").collect();
   println!("firstName is {}",tokens[0]);
   println!("lastname is {}",tokens[1]);
   println!("company is {}",tokens[2]);

   let n1 = "Tutorials".to_string();

   for n in n1.chars(){
    println!("{}",n);
   }

   let w1 = "Sameer".to_string();
   let w2 = "Parmar".to_string();

   let w3 = w1+&w2;
   println!("{}",w3);

   let w4 = "Sameer".to_string();
   let w5 = "Parmar".to_string();
   let w6 = format!("{} {}",w4,w5);
   println!("{}",w6);

}