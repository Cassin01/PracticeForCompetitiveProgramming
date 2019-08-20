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
    let a_s: Vec<i64> =
        (0..n).map(|_| input()).collect();
    let mut a = Vec::new();
    a.push(0);
    for i in a_s {
        a.push(i);
    }
    let mut b = vec![0; n+1];
    for i in (1..n+1).rev() {
        let mut sum =  0;
        let mut j = i+i;
        while j <= n {
            sum ^= b[j];
            j += i;
        }
        b[i] = sum^a[i];
    }
    let mut ans = vec![];
    for (k, &i) in b.iter().enumerate() {
        if i == 1 {
            ans.push(k);
        }
    }
    println!("{}", ans.len());
    for i in ans {
        println!("{}", i);
    }
}