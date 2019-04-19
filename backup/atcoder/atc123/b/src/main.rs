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

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn dd(a: usize, s: usize) -> usize {
    if (a + s)% 5 == 0 {
        return a + s;
    } else {
        return (5 - (a + s) % 5) + a + s;
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }
    let mut v = Vec::new();
    v.push(a);
    v.push(b);
    v.push(c);
    v.push(d);
    v.push(e);

    let mut min = 644;
    for i in 0..5 {
        let mut sum = 0;
        for j in 0..5 {
            if i == j {
                continue;
            } else {
                sum = sum + v[j];
                while sum % 10 != 0{
                    sum+=1;
                }
            }
        }
        sum+=v[i];
        if min >= sum {
            min = sum;
        }
    }

    println!("{}", min);
}