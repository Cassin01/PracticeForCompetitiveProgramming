/*
秒単位の時間 S が与えられるので、h:m:s の形式へ変換して出力してください。ここで、h は時間、m は 60 未満の分、s

は 60 未満の秒とします。
Input

S

が１行に与えられます。
Output

h、m、s

を :（コロン）区切りで１行に出力してください。数値が１桁の場合、0 を付けて２桁表示をする必要はありません。
Constraints

    0≤S<86400

Sample Input

46979

Sample Output

13:2:59
*/

fn main() {
    let s = {
        // バッファを確保
        // 一行読む。失敗を無視
        let mut s = String::new();
        // 改行コードが末尾にくっついてくるので削る
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    };

    println!("{}", s);
}