fn main() {
    let array = [4, 6, 2, 1, 9, 63, -134, 566];
    println!("* {:?}        -> min = {}, max = {}", array, minimum(&array), maximum(&array))
}

fn minimum(arr: &[i32]) -> i32 {
    *arr.iter().min().expect("REASON")
}

fn maximum(arr: &[i32]) -> i32 {
    *arr.iter().max().expect("REASON")
}

// fn minimum(arr: &[i32]) -> i32 {
//     *arr.iter().min().unwrap()
// }
// fn maximum(arr: &[i32]) -> i32 {
//     *arr.iter().max().unwrap()
// }
