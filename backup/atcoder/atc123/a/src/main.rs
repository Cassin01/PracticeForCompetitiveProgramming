macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

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

    ($iter:expr, u641) => {
        read_value!($iter, u64) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn ds(a: u64, b: u64, k:u64) -> bool {
    if a > b {
        return a - b > k;
    } else {
        return b - a > k;
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
        k: u64,
    }

    if ds(a,b,k) {
        println!(":(");
        return;
    } else if ds(b,c,k) {
        println!(":(");
        return;
    } else if ds(c,d,k) {
        println!(":(");
        return;
    } else if ds(d,e,k) {
        println!(":(");
        return;
    } else if ds(e,a,k) {
        println!(":(");
        return;
    } else {
        println!("Yay!");
        return;
    }
}