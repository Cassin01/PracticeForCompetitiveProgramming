fn rate(number: f64) -> f64 {
    let sqrt_n: f64 = (number as f64).sqrt() as f64;
    let log_n:  f64 = (number as f64).log10() as f64;
    //sqrt_n / log_n
    log_n / number
}


fn main() {
    for i in -100..100 {
        /*
        let mut output = String::new();
        for _j in 0..(rate((i * 1000) as f64) as i64) {
            output.push('#');
        }
        println!("{}", output);
        */

        println!("{}", rate(i as f64) as f64);
    }
}
