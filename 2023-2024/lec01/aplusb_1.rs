use std::io::Read;

fn main() {
    let mut buf = String::new();
    if let Ok(_) = std::io::stdin().read_to_string(&mut buf) {
        let mut iter = buf.split_whitespace();
        if let Some(s1) = iter.next() {
            if let Some(s2) = iter.next() {
                if let Ok(v1) = s1.parse::<i32>() {
                    if let Ok(v2) = s2.parse::<i32>() {
                        if let Some(res) = v1.checked_add(v2) {
                            println!("{}", res);
                        }
                    }
                }
            }
        }
    }
}
