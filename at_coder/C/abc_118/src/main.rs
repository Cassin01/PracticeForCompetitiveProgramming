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
        n: usize,
        a_s: [usize; n],
    }
    let mut b_s = a_s;
    let mut min = 1000000000;
    let mut min2 = 1000000000;
    let mut min_index = 0;
    let mut min2_index = 0;
    for i in 0..b_s.len() {
        if b_s[i] <= min {
            min2 = min;
            min2_index = min_index;
            min = b_s[i].clone();
            min_index = i;
        } else if b_s[i] > min && b_s[i] <= min2 {
            min2 = b_s[i].clone();
            min2_index = i;
        }
    }
    loop {
        let x = min2 / (min * 2) + 1;
        for (i, b) in b_s.iter_mut().enumerate() {
            if i != min_index {
                if b >= &mut (x * min) {
                    *b -= x * min;
                } else {
                    *b = 0;
                }
            }
        }
        if b_s[min2_index] == 0 {
            min2 = 1000000000;
            let mut flag = 0;
            for (i, b) in b_s.iter().enumerate() {
                if i != min_index && b != &0 {
                    flag = 1;
                    if b > &0 && b <= &min2 {
                        min2 = b.clone();
                    }
                }
            }
            if flag == 0 {
                println!("{}", b_s[min_index]);
                return;
            }
        }
        else if b_s[min2_index] < b_s[min_index] {
            min = b_s[min2_index].clone();
            min2 = b_s[min_index].clone();

            let tmp = min2_index;
            min2_index = min_index;
            min_index = tmp;
        }
    }
}