#![allow(unused_mut)]
#![allow(non_snake_case)]

#![allow(unused_imports)]
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp::{max, min};

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
    let mut a: Vec<i64> =
        (0..n).map(|_| input()).collect();
    a[0]-=1;
    for i in 1..n {
        if a[i-1] <= a[i] - 1 {
            a[i]-=1;
        }
    }
    for i in 1..n {
        if a[i-1] <= a[i] {
            continue;
        }
        println!("No");
        return;
    }
    println!("Yes");
}