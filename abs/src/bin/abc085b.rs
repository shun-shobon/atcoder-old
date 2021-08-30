use proconio::input;

fn main() {
    input! {
        n: u32,
        mut d: [u32; n],
    }

    d.sort_unstable();
    d.dedup();

    let ans = d.len();

    println!("{}", ans);
}
