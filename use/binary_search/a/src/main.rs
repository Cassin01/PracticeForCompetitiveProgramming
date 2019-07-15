fn main() {
    let vs = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    let v = binary_search(&vs, 51);
    println!("{}", v);
}

// 普通の二分探索
// keyのindexを返す(ない場合は-1)
fn binary_search(vs: &Vec<i64>, key: i64) -> i64 {
    let mut left: i64 = 0;
    let mut right: i64 = vs.len() as i64 - 1;
    while right >= left {
        let mid = (left + (right - left) / 2) as usize;
        if vs[mid] == key {
            return mid as i64;
        } else if vs[mid] > key {
            right = mid as i64 - 1;
        } else if vs[mid] < key {
            left = mid as i64 + 1;
        }
    }
    -1
}