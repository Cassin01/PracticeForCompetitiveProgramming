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

pub fn permutations(size: usize) -> Permutations {
    Permutations { idxs: (0..size).collect(), swaps: vec![0; size], i: 0 }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

fn v2int(vs: Vec<usize>) -> usize {
    let mut ans = 0;
    for (k, v) in vs.into_iter().enumerate() {
        ans += 10_usize.pow(k as u32) * v;
    }
    ans
}

fn int2v(mut n: usize) -> Vec<usize> {
    let mut vs = Vec::new();
    while n > 0 {
        vs.push(n % 10);
        n /= 10;
    }
    vs
}

fn allin(vs: &Vec<usize>) -> bool {
    let mut ls = [false; 3];
    for v in vs.into_iter() {
        if v == &3 {
            ls[0] = true;
        } else if v == &5 {
            ls[1] = true;
        } else if v == &7 {
            ls[2] = true;
        }
    }
    for l in ls.into_iter() {
        if !l {
            return false;
        }
    }
    true
}

fn dfs(mut vs: Vec<usize>, n: usize) -> usize {
    if v2int(vs.clone()) > n {
        0
    } else {
        let mut ret = if allin(&vs) {1} else {0};
        for c in [7, 5, 3].into_iter() {
            let mut new_vs = vs.clone();
            new_vs.push(*c);
            ret += dfs(new_vs, n);
        }
        ret
    }
}


fn main() {
    input! {
        n: usize,
    }
    println!("{}", dfs(int2v(0), n));
}