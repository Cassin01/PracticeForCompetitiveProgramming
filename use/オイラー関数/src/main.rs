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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// 素因数分解をします
// 試し割り法
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

// A: オイラーのトーシェント関数
// B: 愚直に計算
struct A;
struct B;

trait EulersTotient {
    fn eulers_totient(n: usize) -> usize;
}

impl EulersTotient for A {
    fn eulers_totient(n: usize) -> usize {
        let mut s = 1.;
        for (k, _) in trial_division(n) {
            s *= 1. - 1. / k as f64
        }
        (s * n as f64) as usize
    }
}

impl EulersTotient for B {
    fn eulers_totient(n: usize) -> usize {
        (0..n).filter(|x| gcd(*x, n) == 1).count()
    }
}

fn main() {
    input! {
        n: usize,
    }
    assert_eq!(A::eulers_totient(n), B::eulers_totient(n));
}
