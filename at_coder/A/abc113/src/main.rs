fn main() {
    let s = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned() // 改行コードが末尾にくっついてくるので削る
    };

    let (u, v) = {
            let mut ws = s.split_whitespace(); // 空白区切りの単語に分解する
            let u: i32 = ws.next().unwrap().parse().unwrap();
            let v: i32 = ws.next().unwrap().parse().unwrap();
            (u, v)
    };

    println!("{}", u + v / 2);
}