fn main() {
    println!("{}", find_nb(9))
}

fn find_nb(m: u64) -> i32 {
    let mut n: u64 = 0;
    let mut sum: u64 = 0;
    while sum < m {
        n += 1;
        sum += n.pow(3);
    }
    if sum == m {
        n as i32
    } else {
        -1
    }
}

// fn find_nb(n: u64) -> i32 {
//     let mut sum = 0_u64;
//     let l = (0_u64..).take_while(|&x| {sum+=x.pow(3); sum<n}).count() as i32;
//     if sum==n {l}
//     else {-1}
// }
