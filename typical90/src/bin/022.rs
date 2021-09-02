use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let gcd = num::integer::gcd(num::integer::gcd(a, b), c);

    let ans = a / gcd + b / gcd + c / gcd - 3;
    println!("{}", ans);
}
