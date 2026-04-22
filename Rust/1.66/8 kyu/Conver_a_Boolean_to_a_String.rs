fn main() {
    println!("The value is {}", boolean_to_string(true))
}

fn boolean_to_string(b: bool) -> String {
    if b == true { "true".to_string() } else { "false".to_string() }
}

// fn boolean_to_string(b: bool) -> String {
//     b.to_string()
//     /
//     format!("{}", b)
// }
