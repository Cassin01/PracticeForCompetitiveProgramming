fn main() {
    let n = 7;
    let mut ans = Vec::new();
    for x in 2..n {
        let mut a = 1;
        for m in 1..n {
                a = a * x % n;
            if a == 1 {
                if m == n - 1 {
                    ans.push(x);
                } else {
                    break;
                }
            }
        }
    }
    println!("{:?}", ans);
}
