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
        C: [[i64; 3]; 3],
    }

    let mut ok = false;
    for a1 in 0..101 {
        let b1 = C[0][0] - a1;
        let b2 = C[0][1] - a1;
        let b3 = C[0][2] - a1;
        for  a2 in 0..101 {
            for  a3 in 0..101 {
                if
                    C[1][0] == a2 + b1 &&
                    C[1][1] == a2 + b2 &&
                    C[1][2] == a2 + b3 &&
                    C[2][0] == a3 + b1 &&
                    C[2][1] == a3 + b2 &&
                    C[2][2] == a3 + b3 {
                    ok = true;
                }
            }
        }
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}