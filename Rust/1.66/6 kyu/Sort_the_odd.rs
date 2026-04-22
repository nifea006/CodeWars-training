fn main() {
    println!("{:?}", sort_array(&[5, 3, 2, 8, 1, 4]))
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut result = arr.to_vec();
    let odd_numbers: Vec<i32> = arr.iter().filter(|x| *x % 2 != 0).collect();
    odd_numbers.sort_unstable();
}