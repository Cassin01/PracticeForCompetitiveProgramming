fn main() {
    for i in 1..12 {
        let r = 10.0_f64.powf(i as f64 / 10.0);
        let x = f64::sqrt(2.0 * r);
        println!("{}dB {}", i, x);
    }
}
