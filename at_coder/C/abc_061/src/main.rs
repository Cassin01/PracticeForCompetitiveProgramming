use std::io::prelude::*;

fn input<T>() -> T
    where T: std::str::FromStr {
    let stdin = std::io::stdin();
    let token: String = stdin
        .lock()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn main() {
    let n: usize = input();
    let k: usize = input();
    let mut abs: Vec<(i32, usize)> =
        (0..n).map(|_| (input(), input())).collect();
    abs.sort_by_key(|c| c.0);
    let mut sum: usize = 0;
    for ab in abs.iter() {
        if sum + ab.1 >= k {
            println!("{}", ab.0);
            return;
        } else {
            sum += ab.1
        }
    }
}