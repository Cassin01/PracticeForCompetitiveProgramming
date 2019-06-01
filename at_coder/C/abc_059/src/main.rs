#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp::{max, min};

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
        (0..n).map(|_| input()).collect();
    let mut ans = std::i64::MAX;
    for &s in [1, -1].iter() {
        let mut sign = s;
        let mut sum = 0;
        let mut lans = 0;
        for i in 0..n {
            sum += a_s[i];
            // 意図したモノでない
            if sum * sign <= 0 {
                lans += (sum - sign).abs();
                sum = sign;
            }
            sign *= -1;
        }
        ans = min(ans, lans);
    }
    println!("{}", ans);
}