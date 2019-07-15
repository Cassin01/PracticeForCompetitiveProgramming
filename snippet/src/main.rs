use std::io::{BufRead, BufReader, BufWriter, Write};

fn read() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut ret = Vec::new();
    for result in BufReader::new(std::fs::File::open("./01.rs")?).lines() {
        let l = result?;
        ret.push(l);
    }
    Ok(ret)
}

fn write(ret: Vec<String>) {
    let mut f = BufWriter::new(std::fs::File::create("./01.json").unwrap());
    for i in ret {
        f.write(i.as_bytes()).unwrap();
    }
}

fn main() {
    if let Ok(lines) = read() {
        let mut ss = Vec::new();
        for line in lines {
            let mut s = String::new();
            s.push('"');

            let mut cnt = 0;
            let mut f = true;
            for c in line.chars() {
                if c != ' ' {
                    f = false;
                }
                if f {
                    if cnt == 0 {
                        s.push('\\');
                    } else if cnt == 1 {
                        s.push('t');
                    }
                } else {
                    if c == '"' {
                        s.push('\\');
                        s.push('"');
                    } else {
                        s.push(c);
                    }
                }
                cnt += 1;
                cnt %= 4;
            }

            s.push('"');
            s.push(',');
            s.push('\n');
            ss.push(s);
        }
        if !ss.is_empty() {
            let len = ss.len();
            ss[len - 1].pop();
            ss[len - 1].pop();
        }
        write(ss);
    } else {
        return;
    }
}
