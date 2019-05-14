#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
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

#[allow(dead_code)]
fn min_index(vs: Vec<usize>) -> (usize, usize) {
    let mut min = (std::usize::MAX, 0);
    for (i, v) in vs.iter().enumerate() {
        if v < &min.1 {
            min = (i, *v);
        }
    }
    min
}

fn main() {
    input! {
        n: usize,
        s: chars,
    }
    let mut from_w = vec![0; n];
    let mut from_e = vec![0; n];
    let mut sum=0;
    for i in 0..n {
        from_w[i] = sum;
        if s[i] == 'W' {
            sum+=1;
        }
    }
    sum=0;
    for i in (0..n).rev() {
        from_e[i] = sum;
        if s[i] == 'E' {
            sum+=1;
        }
    }
    let mut ss = vec![0; n];
    for i in 0..n {
        ss[i] = from_e[i] + from_w[i];
    }
    println!("{}", ss.iter().min().unwrap());
}