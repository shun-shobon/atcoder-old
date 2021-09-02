use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [(usize, usize); n],
        m: usize,
        q: [(usize, usize); m],
    }

    let mut ans1 = t
        .iter()
        .scan(0, |sum, (c, v)| {
            *sum += if *c == 1 { *v } else { 0 };
            Some(*sum)
        })
        .collect::<Vec<usize>>();
    ans1.insert(0, 0);
    let mut ans2 = t
        .iter()
        .scan(0, |sum, (c, v)| {
            *sum += if *c == 2 { *v } else { 0 };
            Some(*sum)
        })
        .collect::<Vec<usize>>();
    ans2.insert(0, 0);

    for (l, r) in q {
        let ans1 = ans1[r] - ans1[l - 1];
        let ans2 = ans2[r] - ans2[l - 1];

        println!("{} {}", ans1, ans2);
    }
}
