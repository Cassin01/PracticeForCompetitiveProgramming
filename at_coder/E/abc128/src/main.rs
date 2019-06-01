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
    let q: usize = input();
    let mut stx_s: Vec<(i64, i64, i64)> =
        (0..n).map(|_| (input(), input(), input())).collect();
    let mut ds: Vec<i64> =
    (0..q).map(|_| input()).collect();
    let mut xl = Vec::new();
    let mut xr = Vec::new();
    for stx in stx_s.iter() {
        let l = stx.0 - 1 - stx.2;
        xl.push((l, stx.2));
    }

    xl.sort();

    let mut f = 0;
    for d in ds {
    }
}
