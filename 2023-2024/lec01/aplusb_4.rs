use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let v1 = iter.next().unwrap().parse::<i32>().unwrap();
    let v2 = iter.next().unwrap().parse::<i32>().unwrap();
    let res = v1.checked_add(v2).unwrap();
    println!("{}", res);
}
