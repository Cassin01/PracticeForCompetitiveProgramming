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
    let mut a_s: Vec<i64> =
        (0..n+1).map(|_| input()).collect();
    let mut b_s: Vec<i64> =
        (0..n).map(|_| input()).collect();
    let ds = a_s.clone();

    for i in (0..n).rev() {
        let t = b_s[i];
        b_s[i]-=a_s[i+1];
        if b_s[i] < 0 {
            a_s[i+1] -= t;
            b_s[i] = 0;
        } else {
            a_s[i+1] = 0;
        }

        let t = b_s[i];
        b_s[i]-=a_s[i];
        if b_s[i] < 0 {
            a_s[i] -= t;
            b_s[i] = 0;
        } else {
            a_s[i] = 0;
        }
        //println!("{:?}", a_s);
    }

    let mut sum = 0;
    for i in 0..n+1 {
        sum += ds[i] - a_s[i];
    }
    println!("{}", sum);
}