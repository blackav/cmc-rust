use std::io::Read;

fn solve() -> Result<i32, Box<dyn std::error::Error>> {
    let mut buf = String::new();    
    std::io::stdin().read_to_string(&mut buf)?;
    let mut iter = buf.split_whitespace();
    let x1 = match iter.next() {
        Some(s1) => s1.parse::<i32>()?,
        None => return Err("Missing value".into())
    };
    let x2 : i32 = match iter.next() {
        Some(s2) => s2.parse()?,
        None => return Err("Missing value".into())
    };
    match x1.checked_add(x2) {
        Some(res) => return Ok(res),
        None => return Err("Overflow".into())
    }
}

fn main() {
    println!("{}", solve().expect("failed"));
}
