#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::BTreeSet;
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
fn main() {
    input! {
        n: usize,
        rs: [(i64, i64); n],
        bs: [(i64, i64); n],
    }
    let mut rs = rs;
    let mut bs = bs;

    // x 座標でソート
    rs.sort();
    bs.sort();

    let mut ys = BTreeSet::<i64>::new();
    let  mut ri = 0;  // rs の インデックス
    let mut ans = 0;

    for &(bx, by) in bs.iter() {
        // x座標において青より小さい赤を集める
        while ri < rs.len() && rs[ri].0 < bx {
            ys.insert(rs[ri].1);
            ri += 1;
        }
        let mut cand = -1;
        // y座標において青より小さい最大の赤を求める
        for &y in ys.iter() {
            if y < by && y > cand {
                cand = y;
            }
        }
        if cand >= 0 {
            ans +=1;
            ys.remove(&cand);
        }
    }
    println!("{}", ans);
}
