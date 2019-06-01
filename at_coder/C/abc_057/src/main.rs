#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashSet; use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp::{max, min};
use std::io::prelude::*;

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

fn cnt_digits(mut n: i64) -> i64 {
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

fn main() {
    let n: i64 = input();
    let mut ans = cnt_digits(n);
    let mut a = 1;
    while a * a <= n {
        if n % a == 0 {
            let b = n / a;
            let cur = max(cnt_digits(a), cnt_digits(b));
            if ans > cur {
                ans = cur;
            }
        }
        a+=1;
    }
    println!("{}", ans);
}
