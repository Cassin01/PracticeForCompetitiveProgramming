#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::cmp::{max, min};
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
        N: i64,
        A: [i64; N],
    }
    let mut C: HashMap<i64, i64> = HashMap::new();
    for a in A.iter() {
        if C.contains_key(&a) {
            if let Some(time) = C.get_mut(&a) {
                *time+=1;
            }
        } else {
            C.insert(a.clone(), 1);
        }
        /*
        if let Some(time) = C.get_mut(&a) {
            *time+=1;
        } else {
            C.insert(a.clone(), 1);
        }
        */
    }
    let mut time = 0;
    for c in C.into_iter() {
        if c.0 < c.1 {
            time += c.1 - c.0;
        } else if c.0 > c.1 {
            time += c.1;
        }
    }
    println!("{}", time);
}
