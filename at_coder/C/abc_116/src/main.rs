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

fn dfs(mut h: Vec<i64>, r: usize, l: usize, dp: usize) -> i64 {
    let mut az = true;
    let mut u = r;
    let mut vs = Vec::new();
    for j in r..l {
        h[j] -= 1;
        if h[j] >= 0 {
            az = false;
        } else {
        }
        if h[j] > 0 {
        } else {
            vs.push((u, j));
            u = j+1;
        }
    }
    if az {
        return 0;
    }

    vs.push((u, l));

    let mut cnt = 0;
    for  v in vs {
        //println!("{:?} dp{}", v, dp);
        cnt += dfs(h.clone(), v.0, v.1, dp+1);
    }
    cnt + 1
}

fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }
    let mut res = 0;
    for i in 0..n {
        if i == 0 {
            res += xs[i];
        } else {
            if xs[i-1] < xs[i] {
                res += xs[i] - xs[i - 1];
            }
        }
    }
    println!("{}", res);
}
