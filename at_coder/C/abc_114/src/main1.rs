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

//let my_int = from_str::<int>(my_str);
//let my_int = from_str::<int>(my_string.as_slice());
fn generate_max(keta: u32, n: u32) -> u32 {
    let last =  n / u32::pow(10, keta - 1);
    if last >= 7 {
        3 // 3 or 5 or 7
    } else if last >= 5 {
        2 // 3 or 5
    } else if last >= 3 {
        1 // 3
    } else {
        0
    }
}

fn nC3(n: u32) -> u32 {
    let mut j = 1;
    for i in n-2..n+1 {
        j *= i;
    }
    j / 6
}

fn main() {
    let n: u32 = read();
    let keta = keta_10(n);
    generate_max(keta: , n: u32) -> u32 {
}