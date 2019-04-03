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

fn keta_10(mut n: u32) -> u32 {
    let mut i = 0;
    while n != 0 {
        n /= 10;
        i += 1;
    }
    i
}

fn t3(x: u32) {
    let y = x * 10 + 3;
    t3(y);
    t5(y);
    t7(y);
}

fn t5(x: u32) {
    let y = x * 10 + 5;
    t3(y);
    t5(y);
    t7(y);

}

fn t7(x: u32) {
    let y = x * 10 + 7;
    t3(y);
    t5(y);
    t7(y);
}


fn main() {
    let n: u32 = read();
    let keta = keta_10(n);

}