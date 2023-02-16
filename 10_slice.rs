fn main(){
    let mut arr = [0,1,2,3,4,5,6];
    println!("lentgh is {:?}", arr.len());
    borrowing_arry(&mut arr[1..5]);
    println!("{:?}",&mut arr);
}

fn borrowing_arry(slice: &mut [i32]){
    println!("lentgh is {:?}",slice.len());
    println!("{:?}",slice);
    slice[0] = 101;
}