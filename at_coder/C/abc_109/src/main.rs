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
fn gcd_list(numbers: Vec<usize>) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    if let Some((&head, tail)) = numbers.split_first() {
        // &b に注意
        tail.iter().fold(head, |a,&b| gcd(a, b))
    } else {
        panic!("Can't unwrap!");
    }
}
use std::cmp;

fn main() {
    input! {
        n: u64,
        xl: i64,
        x: [i64; n],
    }
    let mut x = x;
    if x.len() == 1 {println!("{}", i64::abs(x[0] - xl));return;}
    x.sort();
    let mut vv = Vec::new();
    let mut mi = i64::abs(x[0] - xl);
    for i in 1..n {
        mi = cmp::min(i64::abs(x[i as usize]- xl), mi);
        vv.push((x[i as usize] - x[(i-1) as usize]) as usize);
    }
    vv.push(mi as usize);
    println!("{}", gcd_list(vv));
}
