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
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m],
    }
    let mut d = s.iter().map(|c| (c, 0)).collect::<HashMap<_, _>>();
    for i in 0..s.len() {
        //d[&s[i]]+=1;
        if let Some(x) = d.get_mut(&s[i]) {
            *x += 1;
        }
    }
    for j in 0..t.len() {
        if d.contains_key(&t[j]) {
            //d[&t[i]]-=1;
            if let Some(x) = d.get_mut(&t[j]) {
                *x -= 1;
            }
        }
    }
    let mut max = 0;
    for (_, &v) in d.iter() {
        if max <= v {
            max = v;
        }
    }
    println!("{}", max);
}