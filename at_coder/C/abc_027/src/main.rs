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

fn dfs(i: i64, time: i64, n: i64) -> i64 {
    if 2 * i > n && 2 * i + 1 > n {
        time
    } else {
        let x = dfs(2 * i + 1, time + 1, n);
        let y = dfs(2 * i, time + 1, n);
        if x > y {
            y
        } else {
            x
        }
    }

}

fn main() {
    input! {
        n: i64,
    }
    let mut m = n;
    let mut depth = 0;
    while m > 0 {
        depth += 1;
        m /= 2;
    }
    let mut m = 1;
    let mut d = 1;
    while m <= n {
        if (depth % 2 == 0 && d % 2 == 0) || (depth % 2 == 1 && d % 2 == 1) {
            m = 2 * m + 1;
        } else {
            m *= 2;
        }
        d += 1;
    }
    println!("{}", if d % 2 == 0 {"Aoki"} else {"Takahashi"});
}
