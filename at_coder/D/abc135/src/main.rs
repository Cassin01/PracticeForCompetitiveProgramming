// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}
fn main() {
    input! {
        s: chars
    }
    let n = 13;
    let mut dp = vec![0; n];
    dp[0] = 1;
    let md = 1000_000_000 + 7;

    let mut mul = 1;
    for i in (0..s.len()).rev() {
        let mut next_dp = vec![0; n];
        let c = s[i];
        if c == '?' {
            for k in 0..10 {
                for j in 0..n {
                    next_dp[(k * mul +j) % n] += dp[j];
                    next_dp[(k * mul +j) % n] %= md;
                }
            }
        } else {
            let k = c.to_digit(10).unwrap() as usize;
            for j in 0..n {
                next_dp[(k * mul +j) % n] += dp[j];
                next_dp[(k * mul +j) % n] %= md;
            }
        }
        mul *= 10;
        mul %= n;
        dp = next_dp;
    }
    println!("{}",  dp[5]);
}