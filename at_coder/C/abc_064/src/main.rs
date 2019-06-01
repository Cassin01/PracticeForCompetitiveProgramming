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
        n: usize,
        a: [i64; n],
    }
    let mut d = vec![0; 8];
    let mut t = 0;
    for i in a.into_iter() {
    if 1    <= i && i <= 399 {
        d[0]+=1;
    } else if 400  <= i && i <= 799  {
        d[1]+=1;

    } else if 800  <= i && i <= 1199 {

        d[2]+=1;

    } else if 1200 <= i && i <= 1599 {

        d[3]+=1;

    } else if 1600 <= i && i <= 1999 {

        d[4]+=1;

    } else if 2000 <= i && i <= 2399 {

        d[5]+=1;

    } else if 2400 <= i && i <= 2799 {

        d[6]+=1;

    } else if 2800 <= i && i <= 3199 {
        d[7]+=1;
    } else {
        t+=1;
    }

    }
    let mut ds = 0;
    for i in d.into_iter() {
        if i >= 1 {
            ds+=1;
        }
    }
    if t != 0 {
        if ds == 0 {
            println!("{} {}", 1, t);
        } else {
            println!("{} {}", ds, ds + t);
        }
    } else {
        println!("{} {}", ds, ds + t);
    }
}
