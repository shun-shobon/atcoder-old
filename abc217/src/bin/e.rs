use proconio::input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    input! {
        q: usize,
    }

    let mut a: VecDeque<usize> = VecDeque::new();
    let mut b: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    for _ in 0..q {
        input! { q: usize }
        match q {
            1 => {
                input! { x: usize }
                a.push_back(x);
            }
            2 => {
                let ans = if b.is_empty() {
                    a.pop_front().unwrap()
                } else {
                    b.pop().unwrap().0
                };
                println!("{}", ans);
            }
            3 => {
                for v in a.drain(..) {
                    b.push(Reverse(v));
                }
            }
            _ => unreachable!(),
        }
    }
}
