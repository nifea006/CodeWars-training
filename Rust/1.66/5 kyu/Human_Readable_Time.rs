fn main() {
    println!("{}", make_readable(1000))
}

fn make_readable(mut seconds: u32) -> String {
    let mut s = 0_u32;
    let mut m = 0_u32;
    let mut h = 0_u32;
    while seconds > 0 {
        s += 1;
        seconds -= 1;
        while s >= 60 {
            m += 1;
            s -= 60;
        }
        while m >= 60 {
            h += 1;
            m -= 60;
        }
    }
    let result = format!("{:02}:{:02}:{:02}", h, m, s);
    result
}

// fn make_readable(s: u32) -> String {
//     let m=s/60;
//     let s=s%60;
//     let h=m/60;
//     let m=m%60;
//     format!("{:02}:{:02}:{:02}",h,m,s)
// }
// fn make_readable(seconds: u32) -> String {
//     format!("{:02}:{:02}:{:02}", seconds / 3600, seconds % 3600 / 60, seconds % 3600 % 60)
// }
