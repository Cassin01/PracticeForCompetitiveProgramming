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
use std::char;
use std::collections::HashMap;
fn main() {

    input! {
        s: chars,
    }
    let mut m = (0..26).map(|x| (x + b'a') as char).map(|c| (c, 0)).collect::<HashMap<_, _>>();
    for i in 0..s.len() {
        //m[&s[i]]+=1;
        if let Some(st) = m.get_mut(&s[i]) {
            *st+=1;
        }
    }

    let mut x: Vec<char> = Vec::new();

    let v = (0..26).map(|x| (x + b'a') as char).collect::<Vec<_>>();
    for i in 0..v.len() {
        if m[&v[i]] == 0 {
            x.push(v[i]);
        }
    }

    if x.is_empty() {
        println!("None");
    } else {
        x.sort();
        println!("{}", x[0]);
    }
}