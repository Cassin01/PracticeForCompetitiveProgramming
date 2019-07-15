trait MOD<T> {
    // べき乗余計算アルゴリズム
    // a^n mod m を計算する
    // "バイナリ法"
    fn pow_mod(a: T, n: T, m: T) -> T;
}

impl MOD<i64> for i64 {
    fn pow_mod(mut a: i64, mut n: i64, m: i64) -> i64 {
        let mut res: i64 = 1;
        while n > 0 {
            if n & 1 == 1 {
                res = res * a % m;
            }
            a = a * a % m;
            n >>= 1;
        }
        res
    }
}

impl MOD<usize> for usize {
    fn pow_mod(mut a: usize, mut n: usize, m: usize) -> usize {
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
}
