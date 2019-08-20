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
    let a: Vec<i64> =
        (0..n).map(|_| input::<i64>() * 2).collect();
    let mut a2 = 0;
    for i in 0..n {
        if i%2==0 {
            a2+=a[i];
        } else{
            a2-=a[i];
        }
    }
    println!("{}", a2/2);
    let mut t = a2/2;
    for i in 0..n-1{
        t = a[i] - t;
        println!("{}", t);
    }
}
/*
a/2 + b/2 = 2
b/2 + c/2 = 2
c/2 + a/2 = 4

a + b = 2 * 2
b + c = 2 * 2
c + a = 4 * 2

a + b = 3 * 2
b + c = 8 * 2
c + d = 7 * 2
d + e = 5 * 2
e + a = 5 * 2

a + b + c + d = 2 * (sum)


*/