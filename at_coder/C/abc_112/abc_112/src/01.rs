fn main() {
    for i in 0..10 {
        for _ in 0..10 {
            println!("{}", i);
            if i == 2 {
                break;
            }
        }
    }
}