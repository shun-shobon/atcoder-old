use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        p: [(usize, usize); q],
    }

    let mut vec = VecDeque::with_capacity(q);

    for (t, x) in p {
        match t {
            1 => vec.push_front(x),
            2 => vec.push_back(x),
            3 => println!("{}", vec[x - 1]),
            _ => unreachable!(),
        }
    }
}
