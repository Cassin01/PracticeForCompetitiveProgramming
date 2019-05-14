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
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut m = HashMap::new();
    for i in 0..n {
        if m.contains_key(&a[i]) {
            if let Some(x) = m.get_mut(&a[i]) {
                *x+=1;
            }
        } else {
            m.insert(a[i], 1);
        }
    }
    let types = m.len();
    let mut v = m.into_iter().map(|c| c.1).collect::<Vec<_>>();
    v.sort();
    v.reverse();
    let mut sum =0;
    if types <= k {
        println!("0");
        return;
    }
    for _ in 0..types-k {
        if let Some(x) = v.pop() {
            sum += x;
        }
    }
    println!("{}", sum);
}