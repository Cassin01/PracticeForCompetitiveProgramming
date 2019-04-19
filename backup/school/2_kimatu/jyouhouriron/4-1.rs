fn main() {
    let ans = 2.0_f64.powf(-1.0)
            + 2.0_f64.powf(-2.0)
            + 2.0_f64.powf(-2.0)
            + 2.0_f64.powf(-3.0);
    println!("{}", ans);

    let ans2 = 2.0_f64.powf(-1.0)
             + 2.0_f64.powf(-3.0)
             + 2.0_f64.powf(-3.0)
             + 2.0_f64.powf(-3.0);
    println!("{}", ans2);
}
