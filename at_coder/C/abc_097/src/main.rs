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
fn pri(i: usize, j: usize, s: &Vec<char>, k: usize) -> String {
    let mut sub_string = String::new();
    let mut time = 0;
    for l in i..j+1 {
        sub_string.push(s[l]);
        time+=1;
        if time > k {
            break;
        }
    }
    sub_string
}

fn main() {
    input! {
        s: chars,
        k: usize,
    }
    let mut v = Vec::new();
    for i in 0..s.len() {
        let mut time = 0;
        for j in i..s.len() {
            v.push(pri(i, j, &s, k));
            time+=1;
            if time > k {
                break;
            }
        }
    }
    v.sort();
    v.dedup();
    println!("{}", v[k-1]);
}