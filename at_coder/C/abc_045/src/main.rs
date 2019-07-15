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
        cs: String
    }
    let is: Vec<u32> = cs.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // 1 << d := pow(2, d)
    let mut ret = 0_u64;
    for i in 0..1 << is.len() - 1 {
        let mut now = 0;
        for j in 0..is.len() {
            // iのj番目ビットが立っているか -> jがiに含まれるか
            if 1 << j & i == 0 {
                let mut vs = Vec::new();
                for k in now..j + 1 {
                    vs.push(is[k]);
                }
                let mut d = 1u64;
                let mut sum = 0u64;
                // for v in vs {
                loop {
                    if let Some(v) = vs.pop() {
                        sum += v as u64 * d;
                    } else {
                        break;
                    }
                    d *= 10;
                }
                now = j + 1;
                ret += sum;
            }
        }
    }
    println!("{}", ret);
}
