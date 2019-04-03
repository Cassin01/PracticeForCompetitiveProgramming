use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn keta_10(mut n: u32) -> u32 {
    let mut i = 0;
    while n != 0 {
        n /= 10;
        i += 1;
    }
    i
}

//let my_int = from_str::<int>(my_str);
//let my_int = from_str::<int>(my_string.as_slice());
fn generate_max(keta: u32, n: u32) -> u32 {
    let last =  n / u32::pow(10, keta - 1);
    println!("{}", last);
    if last >= 7 {
        return gererater(keta, '7');
    } else if last >= 5 {
        return gererater(keta, '5');
    } else if last >= 3 {
        return gererater(keta, '3');
    } else {
        return gererater(keta - 1, '7');
    }
}

fn gererater(keta: u32, end: char) -> u32 {
    let mut st = String::new();
    st.push(end);
    for _ in 0..keta - 4 {
        st.push('7');
    }
    st.push('7');
    st.push('5');
    st.push('3');
    let ri: u32 = st.parse().unwrap();
    ri
}

fn main() {
    let n: u32 = read();
    let keta = keta_10(n);
    let keta_new =  keta_10(generate_max(keta, n));
}