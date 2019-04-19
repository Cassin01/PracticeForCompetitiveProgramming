fn main() {
    let a: f64 = 1.0 / 4.0;
    let b: f64 = 1.0 / 2.0;
    let c: f64 = 1.0 / 4.0;

    let ans =   3.0_f64.log2()
              - (- &a * &a.log2() 
                 - &b * &b.log2()
                 - &c * &c.log2());
    println!("{}", ans);

    let a = 19.0 / 8.0;
    println!("{}", a);
}
