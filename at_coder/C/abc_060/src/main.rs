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
    let t: usize = input();
    let mut ts: Vec<usize> =
        (0..n).map(|_| input()).collect();
    let mut sum = 0;
    for i in 0..ts.len() - 1 {
        if ts[i + 1] - ts[i] > t {
            sum += t;
        } else {
            sum +=  ts[i + 1] - ts[i];
        }
    }
    println!("{}", sum + t);
    // println!("{}", ts.into_iter().sum::<usize>() + n * t as usize);
    /*
    if ts.len() == 1 {
        println!("{}", t);
        return;
    }
    let mut pre: usize = ts[0];
    let mut next: usize;
    let mut total: usize = 0;
    for time in ts.into_iter().skip(1) {
        next = time;
        if next - pre < t { 
            total += next - pre;
        } else { 
            total += t;
        }
    }
    println!("{}", total + t);
    */
}