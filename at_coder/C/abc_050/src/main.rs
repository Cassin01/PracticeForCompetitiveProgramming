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
const MOD: usize = 1e9 as usize + 7;

fn mod_pow(n: usize) -> usize {
    let mut ans = 1;
    for _ in 0..n {
        ans *= 2;
        ans %= MOD;
    }
    ans
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    if n % 2 == 0 {
        let mut hm = HashMap::new();
        for i in 0..n / 2 {
            hm.insert(2 * i + 1, 0);
        }
        for i in a {
            if hm.contains_key(&i) {
                let h = hm.get_mut(&i).unwrap();
                *h += 1;
            } else {
                println!("0");
                return;
            }
        }
        for (_, v) in hm {
            if v == 2 {
            } else {
                println!("0");
                return;
            }
        }
        //println!("{}", usize::pow(2, (n/2) as u32)%MOD);
        println!("{}", mod_pow(n/2));
    } else {
        let mut hm = HashMap::new();
        for i in 0..n / 2 + 1 {
            hm.insert(2 * i, 0);
        }
        for i in a {
            if hm.contains_key(&i) {
                let h = hm.get_mut(&i).unwrap();
                *h += 1;
            } else {
                println!("0");
                return;
            }
        }
        for (k, v) in hm {
            if k ==  0 {
                if v == 1 {
                } else {
                    println!("0");
                    return;
                }
            } else {
                if v == 2 {
                } else {
                    println!("0");
                    return;
                }
            }
        }
        //println!("{}", usize::pow(2, (n/2) as u32)%MOD);
        println!("{}", mod_pow(n/2));
    }
}
