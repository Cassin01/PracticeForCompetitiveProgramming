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

fn max_index(vs: Vec<i64>) -> (usize, i64) {
    let mut max = (0, std::i64::MIN);
    for (i, v) in vs.iter().enumerate() {
        if v > &max.1 {
            max = (i, *v);
        }
    }
    max
}

fn main() {
    let n: usize = input();
    let mut  a_s: Vec<i64> =
        (0..n).map(|_| input()).collect();

    let b = a_s.clone();
    let (u, _) = max_index(a_s.clone());
    a_s.remove(u);
    let (mut v, _) = max_index(a_s);
    if v >= u {
        v+=1;
    }
    for i in 0..n {
        if u != i {
            println!("{}", b[u]);
        } else {
            println!("{}", b[v]);
        }
    }
}