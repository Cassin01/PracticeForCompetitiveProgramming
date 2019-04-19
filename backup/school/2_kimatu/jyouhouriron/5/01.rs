fn main() {
    let a = 0.25_f64;
    let b = 0.21_f64;
    let c = 0.18_f64;
    let d = 2.14_f64;
    let e = 0.12_f64;
    let f = 0.10_f64;
    let e: f64 = &a.log2() * &a
               * &b.log2() * &b
               * &c.log2() * &c
               * &d.log2() * &d
               * &e.log2() * &e
               * &f.log2() * &f;
    println!("{}", e);
}
