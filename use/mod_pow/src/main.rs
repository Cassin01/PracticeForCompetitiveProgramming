// べき乗余計算アルゴリズム
// a^n mod を計算する
// バイナリ法
fn modpow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut res: usize = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }
    res
}

// こっち遅い
fn mod_pow2(a: usize, b: usize, m: usize) -> usize {
    if b == 0  {
        1
    } else if b % 2 == 0 {
        let d = mod_pow2(a, b/2, m);
        (d * d) % m
    } else {
        (a * mod_pow2(a, b-1, m)) % m
    }
}

fn main() {
    //let x = modpow(777, 779, 1763);
    let x = modpow(371, 149, 899);
    println!("{:b}", 149);
    println!("{}", x);
}
