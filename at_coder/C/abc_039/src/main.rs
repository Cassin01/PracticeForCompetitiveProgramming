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
        ss: chars,
    }
    //Do
    //Re
    //Mi
    //Fa
    //So
    //La
    //Si
    /*
    let xs = String::from("WBWWBWBWBWWBWBWWBWWB");
    let mut ss = Vec::new();
    for s in xs.chars() {
        ss.push(s);
    }
    */

    let vs = ['W', 'B', 'W', 'B', 'W', 'W', 'B', 'W', 'B', 'W', 'B', 'W' ];
    let mut ms = Vec::new();
    for i in 0..vs.len() {
        if vs[i] == 'W' {
            ms.push(i);
        }
    }
    let hh = ["Do", "Re", "Mi", "Fa", "So", "La", "Si"];
    let mut  t = 0;
    for m in ms {
        //println!("k>{}", m);
        let mut c = 0;
        let mut k = m;
        let mut f = true;
        while c < 20 {
            if vs[k] != ss[c] {
                f = false;
            }
            k+=1;
            k %= vs.len();
            c+=1;
        }
        if f {
            println!("{}", hh[t]);
            return;
        }
        t+=1;
    }
}
