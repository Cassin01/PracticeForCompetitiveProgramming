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
    let u: Vec<char> = read::<String>().chars().collect();
    let i = u.len();
    println!("{}", i - erase(u).len());

}

fn erase(mut u: Vec<char>) -> String {
    if u.is_empty() {
        String::from("")
    } else {
        for i in 0..u.len() - 1{
            if u[i] != u[i + 1] {
                u.remove(i);
                u.remove(i);
                return erase(u);
            }
        }
        u.into_iter().collect::<String>()
    }
}