fn ee(n: usize) -> f64 {
    let mut e = 1.0_f64;
    let mut d = 1.0_f64;
    for i in 1..n {
        d *= i as f64;
        e += 1.0 / d as f64;
    }
    e
}

fn main() {
    println!("n = 10 の時{}", ee(10));
    println!("n = 100 の時{}", ee(100));
}
