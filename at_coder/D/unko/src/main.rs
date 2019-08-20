use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone)]
struct Edge {
    to: usize,
    cost: usize
}

// https://atcoder.jp/contests/abc021/submissions/5777748
// ↓
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
// ↑
// https://atcoder.jp/contests/abc021/submissions/5777748

// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
// ↓
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
        }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
// ↑
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e

fn main() {
    // 入力
    let n = input::<usize>(); // 頂点数
    let a = input::<usize>() - 1; // スタート地点
    let b = input::<usize>() - 1; // 目的地
    let m = input::<usize>(); // パス数
    let xys = (0..m) // 隣接ペア
        .map(|_| (input::<usize>()-1, input::<usize>()-1)).collect::<Vec<_>>();

    // 追加部分
    let mut cnt = vec![0; n];
    cnt[a] = 1; // 始点aへの最短経路数は1

    let mut d = vec![usize::max_value(); n];
    d[a] = 0;

    let mut g = vec![vec![]; n];
    for (x, y) in xys {
        g[x].push(Edge {to: y, cost: 1});
        g[y].push(Edge {to: x, cost: 1});
    }

    let mut que = BinaryHeap::new();
    que.push(Rev((0, a)));

    while !que.is_empty() {
        let Rev((v_min_d, v)) = que.pop().unwrap();
        if d[v] < v_min_d {
            continue;
        }
        for e in &g[v] {
            if d[e.to] > d[v] + e.cost {
                d[e.to] = d[v] + e.cost;
                que.push(Rev((d[e.to], e.to)));

                // コスト更新の場合親の最短経路数と等しい
                cnt[e.to] = cnt[v];
            } else if d[e.to] == d[v] + e.cost {

                // コストが等しい場合は最短経路数が足し合わされる
                cnt[e.to] += cnt[v];
            }
        }
    }

    println!("スタート地点から各地点までの最短経路数");
    for i in cnt.iter().enumerate() {
        println!("{:?} ", i, );
    }
    println!("目的地までの最短経路数: {}", cnt[b]);
}
