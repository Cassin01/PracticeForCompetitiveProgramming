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


fn main() {
    input! {
        h: usize,
        w: usize,
        t: [chars; h],
    }
    let mut t = t;


    let mut dp = Vec::new();

    for i in 0..h {
        let mut ok = false;
        for j in 0..w {
            if '.' != t[i][j] {
                ok = false;
                break;
            } else {
                ok = true;
            }
        }
        if !ok {
            let mut tmp = Vec::new();
            for j in 0..w {
                tmp.push(t[i][j]);
            }
            dp.push(tmp);
        }
    }

    let mut dp2: Vec<Vec<char>> = Vec::new();

    if !dp.is_empty() {
        for j in 0..dp[0].len() {
            let mut ok = false;
            for i in 0..dp.len() {
                if '.' != dp[i][j] {
                    ok = false;
                    break;
                } else {
                    ok = true;
                }
            }
            if !ok {
                let mut tmp = Vec::new();
                for i in 0..dp.len() {
                    tmp.push(dp[i][j]);
                }
                dp2.push(tmp);
            }
        }
    }

    if !dp2.is_empty() {
        for i in 0..dp2[0].len() {
            for j in 0..dp2.len() {
                print!("{}",dp2[j][i]);
            }
            println!();
        }
    }
}