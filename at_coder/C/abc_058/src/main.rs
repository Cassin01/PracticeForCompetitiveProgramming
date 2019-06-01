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

fn main() {
    let n: usize = input();
    let mut ss: Vec<String> =
        (0..n).map(|_| input()).collect();

    let mut m = (b'a'..b'z'+1)
                .map(|c| c as char)
                .map(|c| (c, 0)).collect::<HashMap<_, _>>();
    for c in ss[0].chars() {
        let mut x = m.get_mut(&c).unwrap();
        *x += 1;
    }
    for s in ss.into_iter() {
        let mut mt = (b'a'..b'z'+1)
            .map(|c| c as char)
            .map(|c| (c, 0)).collect::<HashMap<_, _>>();
        for c in s.chars() {
            let mut x = mt.get_mut(&c).unwrap();
            *x += 1;
        }
        for y in (b'a'..b'z'+1).map(|c| c as char).into_iter() {
            let x = m.get_mut(&y).unwrap();
            *x = min(x.clone(), mt[&y].clone());
        }
    }
    let mut ans = String::new();
    for v in (b'a'..b'z'+1).map(|c| c as char) {
        for _ in 0..m[&v] {
            ans.push(v);
        }
    }
    println!("{}", ans);
}
