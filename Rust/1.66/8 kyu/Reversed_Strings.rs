fn main() {
    println!("{}", solution("world"));
}

fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}
