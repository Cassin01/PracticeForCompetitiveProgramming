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
use std::cmp::*;
fn main() {
    input! {
        n: usize,
        k: usize,
        s: chars,
    }
    let mut starts = vec![];
    let mut ends = vec![];
    if s[0] == '0' {
        starts.push(0);
        ends.push(0);
    }
    for i in 0..s.len() {
        if s[i] == '1' {
            if i == 0 || s[i - 1] == '0' {
                starts.push(i);
            }
            if i == n - 1 || s[i + 1] == '0' {
                ends.push(i);
            }
        }
    }
    if s[n - 1] == '0' {
        starts.push(n - 1);
        ends.push(n - 1);
    }

    if starts.len() == 0 {
        println!("{}", n);
        return;
    }
    let mut ans = 0;
    for i in 0..starts.len() {
        ans = max(ends[min(i + k, ends.len() - 1)] - starts[i] + 1, ans);
    }
    println!("{}" , ans);
}