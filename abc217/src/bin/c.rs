use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut ans = vec![0; n];

    for (i, p) in p.iter().enumerate() {
        ans[*p - 1] = i + 1;
    }

    for ans in ans {
        print!("{} ", ans);
    }
    println!();
}
