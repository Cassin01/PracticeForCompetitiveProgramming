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
        s: chars,
        t: chars
    }
    let mut m: HashMap<char, char> = HashMap::new();
    for i in 0..s.len() {
        if m.contains_key(&t[i]) {
            if m[&t[i]] != s[i] {
                println!("No");
                return;
            }
        } else {
            m.insert(t[i], s[i]);
        }
    }
    let mut m2: HashMap<char, char> = HashMap::new();
    for i in 0..s.len() {
        if m2.contains_key(&s[i]) {
            if m2[&s[i]] != t[i] {
                println!("No");
                return;
            }
        } else {
            m2.insert(s[i], t[i]);
        }
    }
    println!("Yes");
}
