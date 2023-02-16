pub mod movie {
    pub mod bollywood{
        pub mod motivational{
            pub fn play(name:String){
                println!("Movie is {}",name);
            }
        }
    }
}

use movie::bollywood::motivational::play;
fn main(){
    // movie::play("YJHD".to_string());
    play("YJHD".to_string());
    play("Rockstar".to_string());

}