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
        ss: chars,
    }
    let mut ret = 0;
    let mut b = false;
    let mut no_w = true;
    for i in 0..ss.len() {
        if ss[i] == 'B' {
            if b {
            } else {
                b = true;
                ret+=1;
            }
        } else {
            no_w = false;
            b = false;
        }
    }
    let mut si = 0;
    let len = ss.len();
    if ret > 1 {
        if ss[0] == 'B' {
            si += 1;
        }
        if ss[(len - 1) as usize] == 'B' {
            si += 1;
        }
    } else {
        if ss[0] == 'B' || ss[(len - 1) as usize] == 'B' {
            si += 1;
        }
    }
    if no_w {
        println!("{}", 0);
        return;
    }
    if ret == 0 {
        println!("0");
        return;
    }
    println!("{}", ret * 2 - si);
}
