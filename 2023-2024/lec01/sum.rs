use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", buf.split_whitespace().fold(0i32, |a, b| a.checked_add(b.parse::<i32>().unwrap()).unwrap()));
}
