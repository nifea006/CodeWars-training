fn main() {
    println!("{}", solution("camelCase"))
}

fn solution(s: &str) -> String {
    s.chars().map(|c| if c.is_uppercase() { format!(" {}", c) } else { c.to_string() }).collect()
}