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
        t: chars,
    }
    let mut ass = Vec::new();
    for i in 0..s.len() - t.len() + 1 {
        let mut may_in = true;
        for j in 0..t.len() {
            if s[i + j] == t[j] || s[i + j] == '?' {
            } else {
                may_in = false;
            }
        }
        if may_in == true {
            let mut st = String::new();
            for j in 0..i {
                if s[j] == '?' {
                    st.push('a');
                } else {
                    st.push(s[j]);
                }
            }
            for j in 0..t.len() {
                st.push(t[j])
            }
            for j in i + t.len()..s.len() {
                if s[j] == '?' {
                    st.push('a');
                } else {
                    st.push(s[j]);
                }
            }
            ass.push(st);
        }
    }
    if ass.is_empty() {
        println!("UNRESTORABLE");
    } else {
        ass.sort();
        println!("{}", ass[0]);
    }
}
