#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp::{max, min};
use std::io::prelude::*;

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
        ss: chars,
    }
    let mut ss = ss;
    ss.reverse();
    let x0: Vec<char> = vec!['m', 'a', 'e', 'r', 'd'];
    let x1: Vec<char> = vec!['e', 's', 'a', 'r', 'e'];
    let x2: Vec<char> = vec!['r', 'e', 'm', 'a', 'e', 'r', 'd'];
    let x3: Vec<char> = vec!['r', 'e', 's', 'a', 'r', 'e'];

    let mut t = 0;
    while t < ss.len() {
        if ss[t] == x0[0] {
            for k in 0..x0.len() {
                if t + k >= ss.len() {println!("NO"); return;}
                if ss[t + k] != x0[k] {
                    println!("NO");
                    return;
                }
            }
            t += x0.len();
        } else if ss[t] == x1[0] {
            for k in 0..x1.len() {
                if t + k >= ss.len() {println!("NO"); return;}
                if ss[t + k] != x1[k] {
                    println!("NO");
                    return;
                }
            }
            t += x1.len();
        } else if t + 2 < ss.len() {
            if ss[t + 2] == x2[2] {
                for k in 0..x2.len() {
                    if t + k >= ss.len() {println!("NO"); return;}
                    if ss[t + k] != x2[k] {
                        println!("NO");
                        return;
                    }
                }
                t += x2.len();
            } else if ss[t + 2] == x3[2] {
                for k in 0..x3.len() {
                    if t + k >= ss.len() {println!("NO"); return;}
                    if ss[t + k] != x3[k] {
                        println!("NO");
                        return;
                    }
                }
                t += x3.len();
            } else {
                println!("NO");
                return;
            }
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}


/*
dream er ase

erase r
      dream er
      dream eraser
*/