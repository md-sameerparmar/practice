fn main(){
    let arr = [1;4];
    println!("{:?}",arr);
    // println!("{:?}",arr.len());

    // for i in arr.iter(){
    //     println!("{}",i);
    //     if arr[1] == 1{
    //         break;
    //     }
    // }
        fetch_arr(arr);
}
 fn fetch_arr(mut arr:[i32;4]){
    for i in 0..4{
        arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
 }