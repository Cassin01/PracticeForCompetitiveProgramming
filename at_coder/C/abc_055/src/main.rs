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
    // s
    let n: i64 = input();
    // c
    let m: i64 = input();
    // s を全て食べる
    let res  = m - n * 2;

    //  全部食べれる時
    if res >= 0 {
        println!("{}", n + res / 4);

    // 全部食べれなっかった時
    } else {
        if n - m / 2 >= 0 {
            println!("{}", m / 2);
        }
    }
}