use std::io::*;
use std::str::FromStr;

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
    let (xs, ys, hs) = {
        let mut xs: Vec<i32> = Vec::new();
        let mut ys: Vec<i32> = Vec::new();
        let mut hs: Vec<i32> = Vec::new();
        for _ in 0..n {
            let s = {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                s.trim_right().to_owned()
            };
            let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
            xs.push(ws.next().unwrap().parse().unwrap());
            ys.push(ws.next().unwrap().parse().unwrap());
            hs.push(ws.next().unwrap().parse().unwrap());
        } (xs, ys, hs) };

    for i in 0..101 {
        let (tf, j) = innor(i, xs.clone(), ys.clone(), hs.clone());
        if tf {
            let hh = hs[0] + i32::abs(xs[0] - i) + i32::abs(ys[0] - j);
            println!("{} {} {}", i, j, hh);
            return;
        }
    }
}

fn innor(i: i32, xs: Vec<i32>, ys: Vec<i32>, hs: Vec<i32>) -> (bool, i32) {
    for j in 0..101 {
        if tester(xs.clone(), ys.clone(), hs.clone(), i, j) {
            return (true, j);
        }
    }
    return (false, 0);
}

fn tester(x: Vec<i32>, y: Vec<i32>, h: Vec<i32>, i: i32, j: i32) -> bool {
    let hh = h[0] + i32::abs(x[0] - i) + i32::abs(y[0] - j);
    for k in 1..x.len() {
        if h[0] < 1 {
            continue;
        }
        if hh != h[k] + i32::abs(x[k] - i) + i32::abs(y[k] - j) {
            return false;
        }
    }
    return true;
}