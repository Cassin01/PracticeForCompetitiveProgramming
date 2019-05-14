#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::{max, min};
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
fn max_index(vs: Vec<usize>) -> (usize, usize) {
    let mut max = (0, 0);
    for (i, v) in vs.iter().enumerate() {
        if v > &max.1 {
            max = (i, *v);
        }
    }
    max
}

fn solve(l: usize, r: usize, max: usize, v: Vec<usize>) {
    let raw_d = if v[l] > v[r] {
        v[l] - v[r]
    } else {
        v[r] - v[l]
    };
    if raw_d % 2 == 0 {
        println!("{}", (raw_d + 1)/ 2 + max - std::cmp::max(v[l], v[r]));
    } else {
        println!("{}", (raw_d + 1)/ 2 + max - std::cmp::max(v[l], v[r]) + 1);
    }
}

fn main() {
    input! {
        v: [usize; 3],
    }
    let max = max_index(v.clone());
    if max.0==0 {
        solve(1,2, max.1, v.clone());
    } else if max.0 == 1 {
        solve(0,2, max.1, v.clone());
    } else {
        solve(0,1, max.1, v.clone());
    }
}