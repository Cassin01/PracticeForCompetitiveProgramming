
fn main() {
    println!("H(Y | X) {}",
          f(1.0 / 16.0, 4.0 / 16.0)
        + f(2.0 / 16.0, 4.0 / 16.0)
        + f(4.0 / 16.0, 1.0 / 16.0)
    );
    println!("H(X | Y) {}",
          f2(1.0 / 16.0, 2.0 / 16.0 , 4.0 / 16.0)
        + f2(4.0 / 16.0, 4.0 / 16.0, 1.0 / 16.0)
    );
}

fn f(b: f64, c: f64) -> f64 {
    let d = &b + &c;
    let r = b / &d;
    let l = c / &d;
    - d * (&r.log2() * &r + &l.log2() * &l)
}

fn f2(a: f64, b: f64, c: f64) -> f64 {
    let d = &a + &b + &c;
    let s = a / &d;
    let r = b / &d;
    let l = c / &d;

    - d * (&r.log2() * &r
         + &l.log2() * &l
         + &s.log2() * &s)
}
