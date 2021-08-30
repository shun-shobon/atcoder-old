use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }

    let mut ans = 0;

    for a in 0..=a {
        for b in 0..=b {
            for c in 0..=c {
                if a * 500 + b * 100 + c * 50 == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
