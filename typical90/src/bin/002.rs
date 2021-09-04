use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 1 {
        return;
    }

    let mut answers = BTreeSet::new();

    'out: for bit in 0..(1 << n) {
        let mut score = 0;
        for i in 0..n {
            score += if (bit >> i) & 1 == 0 { 1 } else { -1 };
            if score < 0 {
                continue 'out;
            }
        }
        if score != 0 {
            continue;
        }
        let mut s = String::new();
        for i in 0..n {
            s += if (bit >> i) & 1 == 0 { "(" } else { ")" };
        }
        answers.insert(s);
    }

    for ans in answers {
        println!("{}", ans);
    }
}
