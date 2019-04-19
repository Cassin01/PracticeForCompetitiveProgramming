fn main() {
    let p11: f64 = 1.0 / 16.0;
    let p12: f64 = 4.0 / 16.0;
    let p21: f64 = 2.0 / 16.0;
    let p22: f64 = 4.0 / 16.0;
    let p31: f64 = 4.0 / 16.0;
    let p32: f64 = 1.0 / 16.0;

    // H(y|x)
    let px1 = &p11 + &p12;
    let px2 = &p21 + &p22;
    let px3 = &p31 + &p32;

    let pyx11 = &p11 / &px1;
    let pyx12 = &p21 / &px2;
    let pyx13 = &p31 / &px3;
    let pyx21 = &p12 / &px1;
    let pyx22 = &p22 / &px2;
    let pyx23 = &p32 / &px3;

    let hyx = &px1 * (
            - &pyx11 * &pyx11.log2()
            - &pyx21 * &pyx21.log2()
        )
            + &px2 * (
            - &pyx12 * &pyx12.log2()
            - &pyx22 * &pyx22.log2()
        )
            + &px3 * (
            - &pyx13 * &pyx13.log2()
            - &pyx23 * &pyx23.log2()
        );

    println!("{}", hyx);


    // H(x|y)
    let py1 = &p11 + &p21 + &p31;
    let py2 = &p12 + &p22 + &p32;

    let pxy11 = &p11 / &py1;
    let pxy12 = &p12 / &py2;
    let pxy21 = &p21 / &py1;
    let pxy22 = &p22 / &py2;
    let pxy31 = &p31 / &py1;
    let pxy32 = &p32 / &py2;

    let hxy = &py1 * (
            - &pxy11 * &pxy11.log2()
            - &pxy21 * &pxy21.log2()
            - &pxy31 * &pxy31.log2()
        )
            + &py2 * (
            - &pxy12 * &pxy12.log2()
            - &pxy22 * &pxy22.log2()
            - &pxy32 * &pxy32.log2()
        );
    println!("{}", hxy);
}
