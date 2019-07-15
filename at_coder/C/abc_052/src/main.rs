#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::cmp::{max, min};
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

const MOD: i64 = 1e9 as i64 + 7;
fn trial_division(mut n :usize) -> HashMap<usize, usize>{
    let mut primes = HashMap::new();
    let mut i = 2;

    //  n を root n 以下の整数で割り切れるまで割っていく
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            if primes.contains_key(&i) {
                let mut x = primes.get_mut(&i).unwrap();
                *x += 1;
            } else {
                primes.insert(i, 1);
            }
        }
        i+=1;
    }

    // 最後にnが素数になっている場合はそれ自身も素因数に含めて終了
    if n > 1 {
        if primes.contains_key(&n) {
            let mut x = primes.get_mut(&n).unwrap();
            *x += 1;
        } else {
            primes.insert(n, 1);
        }
    }
    primes
}



fn main() {
    let n: usize = input();
    let mut hm = HashMap::new();
    for i in 1..n+1 {
        for (k, v) in trial_division(i) {
            if !hm.contains_key(&k) {
                hm.insert(k, 1);
            } else {
                let x = hm.get_mut(&k).unwrap();
                *x += v;
            }
        }
    }
    let mut cnt = 1;
    for (k, v) in hm {
        cnt *= v + 1;
        cnt %= MOD as usize;
    }
    println!("{}", cnt as i64 % MOD);
}