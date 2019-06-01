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
    let mut map = HashMap::new();
    for i in a.into_iter() {
        if map.contains_key(&i) {
            let x = map.get_mut(&i).unwrap();
            *x += 1;
        } else {
            map.insert(i, 1);
        }
    }
    let mut h = Vec::new();
    for (k, v) in map.into_iter() {
        if v >= 4 {
            h.push(k);
            h.push(k);
        } else if v >= 2 {
            h.push(k);
        }
    }
    if h.len() <= 1 {
        println!("0");
        return;
    } else if h.len() == 2 {
        println!("{}", h[0] * h[1]);
        return;
    }
    let mut ma=h[0] * h[1];
    for i in 0..h.len() {
        for j in 0..h.len() {
            if i != j {
                ma = max(ma, h[i] * h[j]);
            }
        }
    }
    println!("{}", ma);
}