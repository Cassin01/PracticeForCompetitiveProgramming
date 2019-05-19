#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp::{max, min};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

fn dec(a: i64, b: i64) -> f64 {
    if a == 0 && b == 0 {
        0.0
    } else {
        100.0 * (b as f64) / ((a + b) as f64)
    }
}

fn max_index(vs: Vec<f64>) -> (usize, f64) {
    let mut max = (0, 0.0);
    for (i, v) in vs.iter().enumerate() {
        if v > &max.1 {
            max = (i, *v);
        }
    }
    max
}

fn main() {
    input! {
        A: i64,
        B: i64,
        C: i64,
        D: i64,
        E: i64,
        F: i64,
    }
    let mut waters = Vec::new();
    for i in 0..F {
        for j in 0..F {
            if A * i * 100 + B * j * 100 <= F {
                waters.push(A * i * 100 + B * j * 100);
            }
        }
    }
    let mut sugers = Vec::new();
    for i in 0..F {
        for j in 0..F {
            if C * i + D * j <= F {
                sugers.push(C * i + D * j);
            }
        }
    }
    waters.sort();
    waters.dedup();
    waters.remove(0);
    sugers.sort();
    sugers.dedup();

    let mut decs = Vec::new();
    let mut ws = Vec::new();
    let mut ss = Vec::new();

    for i in 0..waters.len() {
        for j in 0..sugers.len() {
            if waters[i] * E >= 100 * sugers[j] && waters[i] + sugers[j] <= F {
                decs.push(dec(waters[i], sugers[j]));
                ws.push(waters[i]);
                ss.push(sugers[j])
            }
        }
    }
    let dec = max_index(decs);
    println!("{} {}", ws[dec.0] + ss[dec.0], ss[dec.0]);
}