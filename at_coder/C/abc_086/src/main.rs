#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::iter::Extend;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
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
fn main() {
    input! {
        n: usize,
        txy: [(i64, i64, i64); n],
    }
    let mut v = vec![(0, 0, 0)];
    let mut d = vec![0;n];
    let mut t = vec![0;n];
    v.extend(&txy);
    for i in 0..n {
        let x = i64::abs(v[i].1 - v[i+1].1);
        let y = i64::abs(v[i].2 - v[i+1].2);
        d[i] = x + y;
        t[i] = i64::abs(v[i].0 - v[i+1].0);
    }
    for i in 0..n {
        if t[i] - d[i] >= 0 && (t[i] - d[i]) % 2 == 0 {
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}