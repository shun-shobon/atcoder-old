use proconio::input;

fn main() {
    input! {
        n: u32,
        mut v: [(i32, i32, i32); n],
    }

    v.insert(0, (0, 0, 0));

    let ans = if v.windows(2).all(|w| {
        let (t, x, y) = w[0];
        let (nt, nx, ny) = w[1];
        let time = nt - t;
        let dist = (nx - x).abs() + (ny - y).abs();
        dist <= time && time % 2 == dist % 2
    }) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
