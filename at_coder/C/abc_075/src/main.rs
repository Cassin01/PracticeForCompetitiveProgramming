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

fn ddd(AB: Vec<(usize, usize)>, drop: usize, N: usize, M: usize) -> bool {
    let mut mp = (0..N).map(|c| (c, 0)).collect::<HashMap<_, _>>();
    for i in 0..M {
        if i != drop  {
            if let Some(x) = mp.get_mut(&AB[i].0) {
                *x += 1;
            }
            if let Some(x) = mp.get_mut(&AB[i].1) {
                *x += 1;
            }
        }
    }
    println!("{:?}", mp);
    let mut c = 0;
    for (_, v) in mp.iter() {
        if c == 0  || c == N - 1 {
            if v >= &1 {
            } else {
                return false;
            }
        } else {
            if v >= &2 {
            } else {
                return false;
            }
        }
        c+=1;
    }
    true
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

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize1, usize1); m],
    }

    let mut ans = 0;
    for i in 0..ab.len() {
        let mut uf = UnionFind::new(n);
        // 同じ集合に入れる
        for j in 0..ab.len() {
            if i != j {
                uf.unite(ab[j].0, ab[j].1);
            }
        }

        let mut ss = HashSet::new();
        for j in 0..n {
            ss.insert(uf.find(j));
        }


        // みんな一つの集合でない -> 橋が消えて別れた
        if ss.len() != 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
