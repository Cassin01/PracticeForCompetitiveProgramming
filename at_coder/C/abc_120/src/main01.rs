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
    let mut u: Vec<char> = read::<String>().chars().collect();
    let i = u.len();
    loop {
        if u.is_empty() {
            println!("{}", i - 0);
            return;
        } else {
            let (c, s) = erase01(u);
            if c == false {
                println!("{}", i - s.len());
                return;
            } else {
                u = s.chars().collect();
            }
        }
    }
}

fn erase01(vs: Vec<char>) -> (bool, String) {
    let mut new_v = String::new();
    let mut b = 'f';
    let mut changed = false;
    for c in vs {
        if b == 'f' {
            new_v.push(c);
            b = c;
        } else if b == c {
            new_v.push(c);
            b = c;
        } else {
            changed = true;
            new_v.pop();
            b = 'f';
        }
    }
    (changed, new_v)
}