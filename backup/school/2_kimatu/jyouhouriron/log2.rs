fn main() {
    // 5 - 1
    let ans = - 0.25 * 0.25_f64.log2()
              - 0.21 * 0.21_f64.log2()
              - 0.18 * 0.18_f64.log2()
              - 0.14 * 0.14_f64.log2()
              - 0.12 * 0.12_f64.log2()
              - 0.10 * 0.10_f64.log2();
    println!("{}", ans);

    // 5 - 2
    let ans2 =   0.25 * 2.0
               + 0.21 * 2.0
               + 0.18 * 3.0
               + 0.14 * 3.0
               + 0.12 * 3.0
               + 0.10 * 3.0;
    println!("{}", ans2);

    // 5 - 3
    let ans3 =   0.25 * 2.0
               + 0.21 * 2.0
               + 0.18 * 3.0
               + 0.14 * 3.0
               + 0.12 * 3.0
               + 0.10 * 3.0;
    println!("{}", ans3);

    // 5 - 4
    let a = 0.25;
    let b = 0.21;
    let c = 0.18;
    let d = 0.14;
    let e = 0.12;
    let f = 0.10;
    println!("5 - 4");

    /*
    let z1 = &a + &b + &c;
    let z2 = &d + &e + &f;
    println!("{}", z1 - z2);

    let z1 = &a + &b;
    let z2 = &c + &d + &e + &f;
    println!("{}", z1 - z2);

    let z1 = &a + &b + &c + &d;
    let z2 = &e + &f;
    println!("{}", z1 - z2);
    */

    let z1 = c + d;
    let z2 = e + f;
    println!("{}", z1 - z2);
}
