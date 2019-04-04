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
    let u: Vec<char> = read::<String>().chars().collect();
    let mut z = 0;
    let mut o = 0;
    for i in u {
        if i == '0' {
            z+=1;
        } else {
            o+=1;
        }
    }
    println!("{}", if o < z { o } else { z } * 2);
}