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

fn ceil(a: usize, b: usize) -> usize {
    ((a as i64 + b as i64 - 1) / b as i64) as usize
}


fn main() {
    input! {
        d: usize,
        g: usize,
        pc: [(usize, usize); d],
    }
    let mut p=Vec::new();
    let mut c=Vec::new();
    for i in 0..d {
        p.push(pc[i].0);
        c.push(pc[i].1);
    }
    let mut min_time = std::usize::MAX;
    for i in 0..1 << d {
        let mut time = 0;
        let mut score = 0;
        // 全部使い切る場合
        for j in 0..d {
            // iに含まれる奴ら
            if 1 << j & i == 0 {
                time += p[j];
                score += c[j] + 100 * (j + 1) * p[j];
            }
        }

        // gより小さかったら
        if score < g {
            let mut p_max = std::usize::MIN;
            for j in 0..d {
                // iに含まれない
                if !(1 << j & i == 0) {
                    p_max = j;
                }
            }
            let r_time = ceil(g - score, (p_max + 1) * 100);
            if r_time >= p[p_max]  {
                continue;
            }
            score += r_time * (p_max + 1) * 100;
            time += r_time;
        }
        use std::cmp;
        min_time = cmp::min(time, min_time);
    }
    println!("{}", min_time);
}
