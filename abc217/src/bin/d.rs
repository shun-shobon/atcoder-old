use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize,
        q: usize,
        query: [(usize, usize); q]
    }

    let mut woods = BTreeSet::new();
    woods.insert(l);

    for (c, x) in query {
        match c {
            1 => {
                woods.insert(x);
            }
            2 => {
                let upper = woods.range(x..).next().unwrap();
                let lower = woods.range(..x).last().unwrap_or(&0);
                let ans = upper - lower;
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
