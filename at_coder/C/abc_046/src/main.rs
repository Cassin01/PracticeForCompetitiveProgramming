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
        n: usize,
        tas: [(i64, i64); n],
    }
    let mut l: i64 = 1;
    let mut r : i64 =  1;
    //println!("");
    for ta in tas {
        let k = max((ta.0 + l - 1)/ta.0, (ta.1 + r - 1)/ta.1);
        l = ta.0 * k;
        r = ta.1 * k;
    }
    println!("{}", l + r);
}


use std::io::prelude::*;

fn input<T>() -> T
    where T: std::str::FromStr {
    let stdin = std::io::stdin();
    let token: String = stdin
        .lock()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn main() {
    let n: usize = input();
    let mut a_s: Vec<i64> =
        (0..n).map(|_| input()).collect();
}
input! {
    n: usize,
    a: [i64; n],
}

let v = vec![1, 2, 3, 4];
// 1 << d := pow(2, d)
for i in 0..1 << v.len() {
    print!("{{");
    for j in 0..v.len() {
        // iのj番目ビットが立っているか -> jがiに含まれるか
        if 1 << j & i == 0 {
            print!("{}", v[j]);
        }
    }
    print!("\n");
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        let p = self.parent[i];
        let p = self.find(p);
        self.parent[i] = p;
        p
    }

    fn unite(&mut self, i: usize, j: usize) {
        let pi = self.find(i);
        let pj = self.find(j);
        self.parent[pi] = pj;
    }
}

