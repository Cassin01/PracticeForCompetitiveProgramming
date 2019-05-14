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
        s:[chars; n],
    }
    let mut d = vec![0_usize; 5];
    for s in s.iter() {
        if s[0] == 'M' {d[0]+=1;}
        if s[0] == 'A' {d[1]+=1;}
        if s[0] == 'R' {d[2]+=1;}
        if s[0] == 'C' {d[3]+=1;}
        if s[0] == 'H' {d[4]+=1;}
    }
    let mut res = 0;
    for first in 0..3 {
        for second in first+1..4 {
            for third in second+1..5 {
                res += d[first] * d[second] * d[third];
            }
        }
    }
    println!("{}", res);
}
