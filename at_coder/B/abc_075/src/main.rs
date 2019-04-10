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
        s: [chars; h],
    }
    let mut d = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                if i == 0 && i == h - 1 {
                    if j == 0 && j == w - 1 {
                        d[i][j]+=1;
                    }
                    else if j == 0 {
                        d[i][j+1]+=1;
                    } else if  j == w - 1 {
                        d[i][j-1]+=1;
                    } else {
                        d[i][j+1]+=1;
                        d[i][j-1]+=1;
                    }
                }
                else if i == 0 {
                    if j == 0 && j == w - 1 {
                        d[i + 1][j]+=1;
                    }
                    else if j == 0 {
                        d[i][j+1]+=1;
                        d[i+1][j+1]+=1;
                        d[i+1][j]+=1;
                    } else if  j == w - 1 {
                        d[i][j-1]+=1;
                        d[i+1][j-1]+=1;
                        d[i+1][j]+=1;
                    } else {
                        d[i][j+1]+=1;
                        d[i][j-1]+=1;
                        d[i+1][j+1]+=1;
                        d[i+1][j-1]+=1;
                        d[i+1][j]+=1;
                    }
                } else if i == h - 1 {
                    if j == 0 && j == w - 1 {
                        d[i-1][j]+=1;
                    } else if j == 0 {
                        d[i][j+1]+=1;
                        d[i-1][j+1]+=1;
                        d[i-1][j]+=1;
                    } else if  j == w - 1 {
                        d[i][j-1]+=1;
                        d[i-1][j-1]+=1;
                        d[i-1][j]+=1;
                    } else {
                        d[i][j+1]+=1;
                        d[i][j-1]+=1;
                        d[i-1][j+1]+=1;
                        d[i-1][j-1]+=1;
                        d[i-1][j]+=1;
                    }
                } else {
                    if j == 0 && j == w - 1 {
                        d[i+1][j]+=1;
                        d[i-1][j]+=1;
                    } else if j == 0 {
                        d[i][j+1]+=1;
                        d[i+1][j+1]+=1;
                        d[i+1][j]+=1;
                        d[i-1][j+1]+=1;
                        d[i-1][j]+=1;
                    } else if  j == w - 1 {
                        d[i][j-1]+=1;
                        d[i+1][j-1]+=1;
                        d[i+1][j]+=1;
                        d[i-1][j-1]+=1;
                        d[i-1][j]+=1;
                    } else {
                        d[i][j+1]+=1;
                        d[i][j-1]+=1;
                        d[i+1][j+1]+=1;
                        d[i+1][j-1]+=1;
                        d[i+1][j]+=1;
                        d[i-1][j+1]+=1;
                        d[i-1][j-1]+=1;
                        d[i-1][j]+=1;
                    }
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                print!("#");
            } else {
                print!("{}", d[i][j]);
            }
        }
        print!("\n");
    }
}
