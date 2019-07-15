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
    let x: i64 = input();
    let mut m: Vec<i64> =
        (0..n).map(|_| input()).collect();
    let mut s = m.clone();
    for i in 0..s.len() - 1 {
        if s[i] + s[i + 1] > x {
            if s[i+1] >= s[i+1] + s[i] - x {
                s[i+1] -= s[i+1] + s[i] - x;
            } else {
                s[i] -= s[i+1] + s[i] - x - s[i+1];
                s[i+1] = 0;
            }
        }
    }
    let mut r = 0;
    for i in 0..s.len() {
        r += m[i] - s[i];
    }
    println!("{}", r);
}
