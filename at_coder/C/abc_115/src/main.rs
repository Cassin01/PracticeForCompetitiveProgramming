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
    let s = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned() // 改行コードが末尾にくっついてくるので削る
    };

    let (n, k) = {
        let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let k: i32 = ws.next().unwrap().parse().unwrap();
        (n, k)
    };

    let mut hs: Vec<i32> = Vec::new();
    for _ in 0..n {
        hs.push(read());
    }

    hs.sort();

    let mut min_h = 1000000000;
    for i in 0..n - k + 1 {
        let min = hs[i as usize];
        let max = hs[(k + i - 1) as usize];
        if min_h > max - min {
            min_h = max - min;
        }
    }

    println!("{}", min_h);
}