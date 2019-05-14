#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
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
        n: i64,
    }
    let mut n = n;
    let mut dd = Vec::new();
    for _ in 0..4 {
        dd.push(n % 10);
        n /= 10;
    }
    dd.reverse();
    // 1 << d := pow(2, d)
    for i in 0..1 << 3 {
        let mut sum = dd[0];
        let mut ans = String::new();
        ans.push(std::char::from_digit(dd[0] as u32, 10).unwrap());
        for j in 0..3 {
            // iのj番目ビットが立っているか -> jがiに含まれるか
            if 1 << j & i == 0 {
                sum+=dd[j+1];
                ans.push('+');
                ans.push(std::char::from_digit(dd[j+1] as u32, 10).unwrap());
            } else {
                sum-=dd[j+1];
                ans.push('-');
                ans.push(std::char::from_digit(dd[j+1] as u32, 10).unwrap());
            }
        }
        if sum == 7 {
            println!("{}=7", ans);
            return;
        }
    }
}