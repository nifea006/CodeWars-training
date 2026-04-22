fn main() {
    println!(
        "There are {} sheep",
        count_sheep(
            &[
                true,
                true,
                true,
                false,
                true,
                true,
                true,
                true,
                true,
                false,
                true,
                false,
                true,
                false,
                false,
                true,
                true,
                true,
                true,
                true,
                false,
                false,
                true,
                true,
            ]
        )
    )
}

fn count_sheep(sheep: &[bool]) -> u8 {
    sheep
        .iter()
        .filter(|&&x| x)
        .count() as u8
}
