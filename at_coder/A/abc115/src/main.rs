use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}


fn main() {
    let u: u32 = read();

    if u == 25 {
        println!("Christmas");
    } else if u == 24 {
        println!("Christmas Eve");
    } else if u == 23 {
        println!("Christmas Eve Eve");
    } else if u == 22 {
        println!("Christmas Eve Eve Eve");
    }
}