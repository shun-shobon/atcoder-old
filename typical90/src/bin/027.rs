use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut names = HashSet::new();

    for (i, s) in s.iter().enumerate() {
        if !names.contains(s) {
            println!("{}", i + 1);
            names.insert(s);
        }
    }
}
