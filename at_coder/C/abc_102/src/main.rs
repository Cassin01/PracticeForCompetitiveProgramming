#![allow(unused_imports)]

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut a=a;
    for i in 0..a.len() {
        a[i] -= i as i64 + 1;
    }
    a.sort();
    let b = {
        if a.len() % 2 == 0 {
            ((a[a.len()/ 2] + a[a.len() / 2 - 1]) / 2) as i64
        } else {
            a[a.len()/ 2]
        }
    };
    let m = {
        a.iter().fold(10000, |min, item| if i64::abs(min-b) < i64::abs((*item)-b) {min} else {*item})
    };
    let ans = a.into_iter().fold(0, |sum, item| sum + i64::abs(item - m));
    println!("{}", ans);
}