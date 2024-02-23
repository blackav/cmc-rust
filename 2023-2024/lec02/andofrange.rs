/*
Given two integers left and right that represent the range [left, right],
return the bitwise AND of all numbers in this range, inclusive.
 */

fn bitwise_and(left : i32, right : i32) -> i32 {
    if left == right {
        return left;
    }
    let mut res = 0;
    let mut b = i32::MIN as u32;

    while (left ^ right) & b as i32 == 0 {
        res |= left & b as i32;
        b >>= 1;
    }
    res
}

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let x = iter.next().unwrap().parse::<i32>().unwrap();
    let y = iter.next().unwrap().parse::<i32>().unwrap();
    println!("{}", bitwise_and(x, y));
}
