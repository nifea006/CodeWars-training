fn main() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11 * 11, 121 * 121, 144 * 144, 19 * 19, 161 * 161, 19 * 19, 144 * 144, 19 * 19];
    println!("{}", comp(a1, a2))
}

fn comp(a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    } else {
        let mut squared: Vec<i64> = a
            .iter()
            .map(|x| x * x)
            .collect();
        squared.sort_unstable();
        b.sort_unstable();
        if squared == b {
            return true;
        } else {
            return false;
        }
    }
}

// fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
//     a = a.iter().map(|x| x.pow(2)).collect();
//     a.sort();
//     b.sort();
//     a == b
// }