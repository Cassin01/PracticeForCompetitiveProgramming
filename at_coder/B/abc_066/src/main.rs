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

fn ch(s: &String, r: &String) -> bool {
    if s == r {
        true
    } else {
        false
    }
}
fn main() {
    input! {
        s: chars,
    }
    let mut s = s;
    s.pop();
    s.pop();
    while !s.is_empty() {
        let mut f = String::new();
        let mut b = String::new();
        for j in 0..s.len() / 2 {
            f.push(s[j].clone());
        }
        for j in s.len() / 2 ..s.len() {
            b.push(s[j].clone());
        }
        if ch(&f, &b) {
            println!("{}", s.len());
            return;
        } else {
            s.pop();
            s.pop();
        }
    }
    println!("{}", 0);
}
