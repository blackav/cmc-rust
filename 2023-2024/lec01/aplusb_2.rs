use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).expect("read failed");
    let mut iter = buf.split_whitespace();
    let s1 = iter.next().expect("value expected");
    let s2 = iter.next().expect("value expected");
    let v1 = s1.parse::<i32>().expect("i32 expected");
    let v2 = s2.parse::<i32>().expect("i32 expected");
    let res = v1.checked_add(v2).expect("overflow");
    println!("{}", res);
}
