use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let ans: u32 = (1..=n)
        .filter(|x| {
            let x = x.to_string().chars().map(|x| x.to_digit(10).unwrap()).sum();
            a <= x && x <= b
        })
        .sum();

    println!("{}", ans);
}
