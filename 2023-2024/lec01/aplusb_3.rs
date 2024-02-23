use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).expect("read failed");
    let mut iter = buf.split_whitespace();
    let v1 = iter.next().expect("value expected").parse::<i32>().expect("i32 expected");
    let v2 = iter.next().expect("value expected").parse::<i32>().expect("i32 expected");
    let res = v1.checked_add(v2).expect("overflow");
    println!("{}", res);
}
