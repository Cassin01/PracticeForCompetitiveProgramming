use std::io::*;
use std::str::FromStr;
use std::collections::HashMap;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let n: u32 = read();
    let s = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned() // 改行コードが末尾にくっついてくるので削る
    };

    let vs: Vec<u32> = {
        let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
        let mut vs = Vec::new();
        for _ in 0..n {
            vs.push(ws.next().unwrap().parse().unwrap());
        }
        vs
    };

    let mut map1: HashMap<u32, u32> = HashMap::new();
    let mut map2: HashMap<u32, u32> = HashMap::new();
    for (i ,v) in vs.iter().enumerate() {
        if i % 2 == 0 {
            if map1.contains_key(&v) {
                let tmp = map1[v].clone();
                map1.insert(v.clone(), tmp + 1);
            } else {
                map1.insert(v.clone(), 1);
            }
        } else {
            if map2.contains_key(&v) {
                let tmp = map2[v].clone();
                map2.insert(v.clone(), tmp + 1);
            } else {
                map2.insert(v.clone(), 1);
            }
        }
    }
    let mut max1 = 0;
    let mut max1b = 0;
    let mut v1 = 0;
    for (k, v) in &map1 {
        if v >= &max1 {
            v1 = k.clone();
            max1b = max1.clone();
            max1 = v.clone();
        }
    }

    let mut max2 = 0;
    let mut max2b = 0;
    let mut v2 = 0;
    for (k, v) in &map2 {
        if v >= &max2 {
            v2 = k.clone();
            max2b = max2.clone();
            max2 = v.clone();
        }
    }

    //println!("{:?}", map1);
    //println!("{:?}", map2);

    let mut ans1 = max1;
    let mut ans2 = max2;
    if v1 == v2 {
        if max1 == max2 {
            if max1b > max2b {
                ans1 = max1b;
            } else {
                ans2 = max2b;
            }
        } else if max1 > max2 {
            ans2 = max2b;
        } else {
            ans1 = max1b;
        }
    }
    println!("{}", n - ans1 - ans2);
}