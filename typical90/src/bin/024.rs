use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let diff = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs() as usize)
        .sum();

    if k >= diff && (k - diff) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
