use std::collections::HashMap;

fn main() {
    let mut s1 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    };

    let mut s2 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    };


    let mut map1: HashMap<char, Vec<usize>> = HashMap::new();
    let mut map2: HashMap<char, Vec<usize>> = HashMap::new();
    for i in 0..26 {
        map1.insert((i + b'a') as char, Vec::new());
        map2.insert((i + b'a') as char, Vec::new());
    }

    let mut vs1 = Vec::new();
    let mut vs2 = Vec::new();

    let mut k = 0;
    while let Some(c1) = s1.pop() {
        if let Some(c2) = s2.pop() {
            if let Some(v) = map1.get_mut(&c1) {
                v.push(k.clone());
            }
            vs1.push(c1);
            vs2.push(c2);
        }
        k += 1;
    }

    for i in 0..vs1.len() {
        if vs1[i] != vs2[i] {
            let tmp_vc1 = vs1[i].clone();
            let tmp_vc2 = vs2[i].clone();
            let mut remaind_vc1 = Vec::new();
            let mut remaind_vc2 = Vec::new();

            if let Some(v) = map1.get_mut(&tmp_vc1) {
                while let Some(j) = v.pop() {
                    remaind_vc1.push(j.clone() as usize);
                }
            }
            if let Some(v) = map1.get_mut(&tmp_vc2) {
                while let Some(j) = v.pop() {
                    remaind_vc2.push(j.clone() as usize);
                }
            }

            for rv in remaind_vc1 {
                vs1[rv] = tmp_vc2.clone();
                if let Some(v) = map1.get_mut(&tmp_vc2) {
                    v.push(rv.clone());
                }
            }
            for rv in remaind_vc2 {
                if let Some(v) = map1.get_mut(&tmp_vc1) {
                    v.push(rv.clone());
                }
                vs1[rv] = tmp_vc1.clone();
            }
        }
    }

    if vs1 == vs2 {
        println!("Yes");
    } else {
        println!("No");
    }
}