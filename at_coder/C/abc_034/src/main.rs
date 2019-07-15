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

fn mod_pow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut res: usize = 1;
    while n > 0 {
            if n & 1 == 1 {
                    res = res * a % m;
                }
            a = a * a % m;
            n >>= 1;
    }
    res
}

fn mod_pow2(a: usize, b: usize, m: usize) -> usize {
    if b == 0  {
        1
    } else if b % 2 == 0 {
        let d = mod_pow2(a, b/2, m);
        (d * d) % m
    } else {
        (a * mod_pow2(a, b-1, m)) % m
    }
}

fn main() {
    input! {
        w: usize ,
        h: usize
    }
    let m:usize = 1000_000_007;

    let sum = w + h - 2;

    let wi = w-1;
    let hi = h-1;
    let mut s1 = 1;
    for i in wi+1..sum+1 {
        s1 *= i;
        s1 %= m;
    }
    for i in 1..hi+1 {
        s1 *= mod_pow2(i, m-2, m) % m;
        s1 %= m;
    }
    println!("{}",  s1 % m);
}
