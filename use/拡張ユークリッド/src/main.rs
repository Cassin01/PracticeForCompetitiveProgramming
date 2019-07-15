#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp::{max, min};

// 拡張ユークリッド互除法
// 入力: 整数 u, v (u > v > 0)
// 出力: u と v の最大公約数 d (=r0) と, s * u + t * v = d を満たす s, t
// 返り値: e := v^{-1} mod 19 を満たす e (=t)
fn euqlid(u: i64, v: i64) -> i64 {
    let mut r0 = u;
    let mut r1 = v;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;
    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 - q * r1;
        let s = s0 - q * s1;
        let t = t0 - q * t1;
        r0 = r1;
        s0 = s1;
        t0 = t1;
        r1 = r;
        s1 = s;
        t1 = t;
    }
    println!("{} * {} + {} + {} = {}", s0, u, t0, v, r0);
    if t0 < 0 {
        t0 + u
    } else {
        t0
    }
}

fn main() {
    assert_eq!(11, euqlid(19, 7));
}
