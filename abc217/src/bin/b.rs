use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: [String; 3],
    }

    let mut set = HashSet::new();
    set.insert("ABC");
    set.insert("ARC");
    set.insert("AGC");
    set.insert("AHC");

    for s in s {
        set.remove(s.as_str());
    }

    for ans in set {
        println!("{}", ans);
    }
}
