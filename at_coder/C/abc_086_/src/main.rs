#![allow(unused_mut)]
#![allow(non_snake_case)]

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
        N: usize,
        A1: [usize; N],
        A2: [usize; N],
    }
    let mut u = vec![0;N];
    let mut d = vec![0;N];
    let mut sum = 0;
    for i in 0..N {
        sum+= A1[i];
        u[i]=sum;
    }
    sum = 0;
    for i in (0..N).rev() {
        sum+= A2[i];
        d[i]=sum;
    }
    let mut max = 0;
    for i in 0..N {
        let l = u[i] + d[i];
        max = std::cmp::max(max, l);
    }
    println!("{}", max);
}