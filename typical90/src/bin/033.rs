use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
    }

    let ans = if h == 1 || w == 1 {
        h * w
    } else {
        ((h + 1) / 2) * ((w + 1) / 2)
    };
    println!("{}", ans);
}
