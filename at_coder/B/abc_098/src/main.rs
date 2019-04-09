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

use std::collections::HashMap;

fn main() {
    input! {
        _n: usize,
        s: chars,
    }
    let mut called_time = Vec::new();
    for i in 1..s.len() {
        let mut m1 = (b'a'..b'z')
                    .map(|c| c as char)
                    .filter(|c| c.is_alphabetic())
                    .map(|c| (c, 0)).collect::<HashMap<_, _>>();
        m1.insert('z', 0);
        let mut m2 = (b'a'..b'z')
                    .map(|c| c as char)
                    .filter(|c| c.is_alphabetic())
                    .map(|c| (c, 0)).collect::<HashMap<_, _>>();
        m2.insert('z', 0);
        let mut v = (b'a'..b'z').map(|c| c as char).collect::<Vec<_>>();
        v.push('z');
        for j in 0..i {
            if let Some(x) = m1.get_mut(&s[j]) {
                *x += 1;
            }
        }
        for j in i..s.len() {
            if let Some(x) = m2.get_mut(&s[j]) {
                *x += 1;
            }
        }
        let mut time = 0;
        for j in 0..v.len() {
            if m2[&v[j]] != 0 && m1[&v[j]] != 0 {
                time+=1;
            }
        }
        called_time.push(time);
    }
    if let Some(x) = called_time.iter().max() {
        println!("{}", x);
    }
}