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
fn main() {
    input! {
        h:  i64,
        w: i64
    }
    if h % 3  == 0 {
        println!("0");
    } else if w % 3 == 0 {
        println!("0");
    } else {
        let mut  mi = std::i64::MAX;
        if h > w {
            let tmp = if h % 3 == 0 {0} else {1};
            mi = std::cmp::min(mi, (tmp % 3) * w);
        } else {
            let tmp = if w % 3 == 0 {0} else {1};
            mi = std::cmp::min(mi, (tmp % 3) * h);
        }
        for i  in 1..h {
            let v = vec![(h - i) * (w / 2), i * w, (h-i) * ((w + 2 - 1) / 2)];
            mi = std::cmp::min(mi,
                v.iter().max().unwrap() - v.iter().min().unwrap());
        }
        for i  in 1..w {
            let v = vec![(w - i) * (h / 2), i * h, (w - i) * ((h + 2 - 1) / 2)];
            mi = std::cmp::min(mi,
                v.iter().max().unwrap() - v.iter().min().unwrap());
        }
        println!("{}", mi);
    }
}
