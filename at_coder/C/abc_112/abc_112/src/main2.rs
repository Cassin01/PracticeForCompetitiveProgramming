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
        }
        (xs, ys, hs)
    };
}

fn solve(h_1: i32, h_2: i32, x_1: i32, x_2: i32, y_1: i32, y_2: i32) {
    // 1 1 1 1
    // not

    // 1 1 1 0
    let c_y = (x_2 - x_1 + y_2 + y_1 - (h_1 - h_2)) / 2;
    if y_1 - c_y <= 0{
        // ok
    }

    // 1 1 0 1
    let c_x = ((x_2 + x_1 + y_2 - y_1) - (h_1 - h_2)) / 2;
    if x_1 - c_x <= 0 {
        // ok
    }

    // 1 1 0 0
    // c_x, c_y

    // 1 0 1 1
    let c_y = ((h_1 - h_2) - (x_2 - x_1 - y_1 - y_2)) / 2;
    if y_2 - c_y <= 0 {
        // ok
    }

    // 1 0 1 0
    // not

    // 1 0 0 1
    // c_x, c_y

    // 1 0 0 0
    let c_x = ((x_2 + x_1 + y_1 - y_2) - (h_1 - h_2)) / 2;
    if y_2 - c_y <= 0 && y_1 - c_y <= 0 && x_1 - c_x <= 0 {
        // ok
    }

    // 0 1 1 1
    let c_x = y_2 - y_1 - x_2 - x_1 - (h_1 - h2);
    if x_2 - c_x {
        // ok
    }

    // 0 1 1 0
    // c_x, c_y

    // 0 1 0 1
    // not

    // 0 0 1 1
    // c_x, c_y

    // 0 0 1 0
}
