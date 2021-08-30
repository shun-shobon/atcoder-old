use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", s.iter().filter(|&n| *n == '1').count());
}
