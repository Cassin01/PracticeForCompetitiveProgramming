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
    let x: usize = input();
    let tmp_cnt = x / 11 * 2;
    let ret = x % 11;
    if ret > 6 {
        println!("{}", tmp_cnt + 2);
    } else if ret > 0 {
        println!("{}", tmp_cnt + 1);
    } else {
        println!("{}", tmp_cnt);
    }
}
