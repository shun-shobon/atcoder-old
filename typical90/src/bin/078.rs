use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }

    let mut v = vec![Vec::new(); n];

    for (a, b) in e {
        v[a].push(b);
        v[b].push(a);
    }

    let ans = v
        .iter()
        .enumerate()
        .filter(|(i, v)| v.iter().filter(|x| *x < i).count() == 1)
        .count();
    println!("{}", ans);
}
