#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        students: [(isize, isize); n],
        goals: [(isize, isize); m],
    }

    for i in 0..n {
        let mut checkpoint = 1;
        let mut distance = (students[i].0 - goals[0].0).abs() as usize
            + (students[i].1 - goals[0].1).abs() as usize;
        for k in 0..m {
            let tmp = (students[i].0 - goals[k].0).abs() as usize
                + (students[i].1 - goals[k].1).abs() as usize;
            if tmp < distance {
                distance = tmp;
                checkpoint = k + 1;
            }
        }
        println!("{}", checkpoint);
    }
}
