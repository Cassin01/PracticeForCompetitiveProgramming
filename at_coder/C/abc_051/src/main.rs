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
    let sx: i64 = input();
    let sy: i64 = input();
    let tx: i64 = input();
    let ty: i64 = input();
    let len_x = tx - sx;
    let len_y = ty - sy;
    for _ in 0..len_y + 0 {
        print!("U");
    }
    for _ in 0..len_x + 0 {
        print!("R");
    }
    for _ in 0..len_y + 0 {
        print!("D");
    }
    for _ in 0..len_x + 1 {
        print!("L");
    }
    for _ in 0..len_y + 1 {
        print!("U");
    }
    for _ in 0..len_x + 1 {
        print!("R");
    }
    print!("D");
    print!("R");
    for _ in 0..len_y + 1 {
        print!("D");
    }
    for _ in 0..len_x + 1 {
        print!("L");
    }
    println!("U");
}

