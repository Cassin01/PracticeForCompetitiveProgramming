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
        h: usize,
        w: usize,
        cs: [chars; h],
    }
    let mut map = vec![vec!['#'; w+2]; h+2];
    let mut ved = vec![vec![false; w+2]; h+2];
    let mut s = (0, 0);
    let mut g = (0, 0);
    for j in 0..h {
        for i in 0..w {
            map[j+1][i+1] = cs[j][i];
            if cs[j][i] == 's' {
                s = (j+1, i+1);
            } else if cs[j][i] == 'g' {
                g = (j+1, i+1);
            }
        }
    }

    let mut stack = Vec::new();
    stack.push(s);
    let mut now = s;
    ved[now.0][now.1] = true;
    let ds = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut flag = false;
    while !stack.is_empty() {
        now = stack.pop().unwrap();

        if now == g {
            flag = true;
            break;
        }

        for d in ds.iter() {
            let  y = (now.0 as i64 + d.0) as usize;
            let  x = (now.1 as i64 + d.1) as usize;

            if map[y][x] != '#' && !ved[y][x] {
                stack.push((y, x));
            }

            ved[now.0][now.1] = true;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No")
    }
}
