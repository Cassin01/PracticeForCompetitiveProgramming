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
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut a = a;
    let mut v = vec![0];
    v.append(&mut a);
    let mut d = vec![0;n+1];
    v.push(0);
    for i in 0..n+1 {
        d[i]=i64::abs(v[i]- v[i+1]);
    }
    for i in 1..n+1 {
        d[i]=i64::abs(v[i]- v[i+1]);
    }
    let sum = d.iter().sum::<i64>();
    for i in 1..n+1 {
        println!("{}", sum - d[i-1] - d[i] + i64::abs(v[i-1] - v[i+1]));
    }
}
