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

    let (n, m) = {
            let s = {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                s.trim_right().to_owned()
            };
            let mut ws = s.split_whitespace();
            let n: usize = ws.next().unwrap().parse().unwrap();
            let m: usize = ws.next().unwrap().parse().unwrap();
            (n, m)
    };
    let mut ms = vec![0; m];

    for _ in 0..n {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_right().to_owned()
        };
        let mut ws = s.split_whitespace();
        let k = ws.next().unwrap().parse().unwrap();
        for _ in 0..k {
            let tmp: i64 = ws.next().unwrap().parse().unwrap();
            ms[(tmp - 1) as usize]+=1;
        }
    }
    let mut res = 0;
    for i in 0..m {
        if ms[i] == n {
            res+=1;
        }
    }
    println!("{}", res);
}