/*
In a town, there are n people labeled from 1 to n. There is a rumor that
one of these people is secretly the town judge.

If the town judge exists, then:

    The town judge trusts nobody.
    Everybody (except for the town judge) trusts the town judge.
    There is exactly one person that satisfies properties 1 and 2.

You are given an array trust where trust[i] = [ai, bi] representing
that the person labeled ai trusts the person labeled bi.
If a trust relationship does not exist in trust array,
then such a trust relationship does not exist.

Return the label of the town judge if the town judge exists
and can be identified, or return -1 otherwise.
 */

fn find_judge(n : i32, data: Vec<(i32, i32)>) -> i32 {
    let mut trust : Vec<(i32, i32)> = vec![(0, 0); n as usize];
    for i in 0..data.len() {
        trust[data[i].0 as usize - 1].0 += 1;
        trust[data[i].1 as usize - 1].1 += 1;
    }
    for i in 0..trust.len() {
        if trust[i].0 == 0 && trust[i].1 == n - 1 {
            return i as i32 + 1;
        }
    }
    -1
}

fn main() {
    println!("{}", find_judge(3, vec![(1, 2), (3, 2)]));
    println!("{}", find_judge(3, vec![(1, 2), (3, 2), (2, 1)]));
}
