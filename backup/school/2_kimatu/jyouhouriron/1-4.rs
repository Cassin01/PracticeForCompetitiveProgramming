fn l2(x: f64) -> f64 {
    x.log2()
}

fn main() {
    let p11: f64 = 1.0 / 16.0;
    let p12: f64 = 4.0 / 16.0;
    let p21: f64 = 2.0 / 16.0;
    let p22: f64 = 4.0 / 16.0;
    let p31: f64 = 4.0 / 16.0;
    let p32: f64 = 1.0 / 16.0;

    let px1 = &p11 + &p12;
    let px2 = &p21 + &p22;
    let px3 = &p31 + &p32;

    let py1 = &p11 + &p21 + &p31;
    let py2 = &p12 + &p22 + &p32;

    // I(X;Y)
    let ixy = &p11 * l2(&p11 / (&px1 * &py1))
            + &p12 * l2(&p12 / (&px1 * &py2))
            + &p21 * l2(&p21 / (&px2 * &py1))
            + &p22 * l2(&p22 / (&px2 * &py2))
            + &p31 * l2(&p31 / (&px3 * &py1))
            + &p32 * l2(&p32 / (&px3 * &py2));
    println!("{}", ixy)
}
