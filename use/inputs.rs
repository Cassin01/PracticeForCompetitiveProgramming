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
    let u: u32 = read();
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
}