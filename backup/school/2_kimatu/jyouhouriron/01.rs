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

    extern crate ndarray;
    use ndarray::{arr2};

    fn main() {
        let matrix = arr2(&[[1, 4],
                            [2, 4],
                            [4, 1]]);
        println!("{:?}", matrix[0][1]);
    }
}
