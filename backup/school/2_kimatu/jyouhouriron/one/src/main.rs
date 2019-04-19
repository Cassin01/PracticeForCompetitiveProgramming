extern crate ndarray;

fn main() {

    let x1: f64 = 5.0 / 16.0;
    let x2: f64 = 6.0 / 16.0;
    let x3: f64 = 5.0 / 16.0;

    let v: f64 =
        -vec![x1, x2, x3].into_iter().map(|x| x.log2() * x).fold(0.0, |sum, x| sum + x);
    println!("1-1 {}", v);

    let y1: f64 = 7.0 / 16.0;
    let y2: f64 = 9.0 / 16.0;

    let v2: f64 =
        -vec![y1, y2].into_iter().map(|x| x.log2() * x).fold(0.0, |sum, x| sum + x);
    println!("1-1 {}", v2);

    let mut vx1: Vec<f64> = Vec::new();
    vx1.push(vf(1.0));
    vx1.push(vf(2.0));
    vx1.push(vf(4.0));

    let mut vx2: Vec<f64> = Vec::new();
    vx2.push(vf(4.0));
    vx2.push(vf(4.0));
    vx2.push(vf(1.0));

    let a1: f64 =
        -vx1.into_iter().map(|x| x.log2() * x).fold(0.0, |sum, x| sum + x);
    let a2: f64 =
        -vx2.into_iter().map(|x| x.log2() * x).fold(0.0, |sum, x| sum + x);
    println!("1-1 {}", a1 + a2);

}

fn vf(i: f64) -> f64 {
    i / 16.0
}
