fn main() {
    let py1: f64 = 7.0 / 16.0;
    let py2: f64 = 9.0 / 16.0;
    let px1: f64 = 5.0 / 16.0;
    let px2: f64 = 6.0 / 16.0;
    let px3: f64 = 5.0 / 16.0;
    let hx = - py1 * py1.log2()
             - py2 * py2.log2();
    let hy = - px1 * px1.log2()
             - px2 * px2.log2()
             - px3 * px3.log2();
    println!("{}", hx);
    println!("{}", hy);
}
