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

fn maxu(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    input! {
        n: usize,
        a: usize,
        xs: [usize; n],
    }
    /*
    let n = 4;
    let a = 8;
    let mut xs = vec![4, 9, 8, 9];
    */

    let max =  xs.iter().max().unwrap();
    let max = maxu(*max, a);

    let mut dp = vec![vec![vec![0; n*max+1]; n+1]; n+1];

    for j in 0..n+1 {
        for k in 0..n+1 {
            for s in 0..n*max+1 {
                if j == 0 && k == 0 && s == 0 {
                    dp[j][k][s] = 1;
                } else if j >= 1 &&  s < xs[(j as i64 -1) as usize] {
                    dp[j][k][s] = dp[j-1][k][s];
                } else if j >= 1 && k >= 1 && s >= xs[(j as i64 -1) as usize] {
                    dp[j][k][s] = dp[j-1][k][s] + dp[j-1][k-1][s-xs[(j as i64 -1) as usize]];
                } else {
                    dp[j][k][s] = 0
                }
            }
        }
    }
    let mut ret: usize =  0;
    for k in 1.. n + 1 {
        ret += dp[n][k][k*a];
    }
    println!("{}", ret);
}
