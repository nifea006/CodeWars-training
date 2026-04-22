fn main() {
    if zero_fuel(60, 25, 2) == true {
        println!("You can do it!")
    } else {
        println!("You won't succeed!")
    }
}

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    if distance_to_pump <= mpg * gallons { true } else { false }
}

// fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
//   distance_to_pump <= mpg * gallons
// }
