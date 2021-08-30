use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s = s.chars().rev().collect::<String>();
    let mut s = s.as_str();

    loop {
        if s.len() == 0 {
            println!("YES");
            return;
        }
        if s.starts_with("maerd") {
            s = &s[5..];
            continue;
        }
        if s.starts_with("remaerd") {
            s = &s[7..];
            continue;
        }
        if s.starts_with("esare") {
            s = &s[5..];
            continue;
        }
        if s.starts_with("resare") {
            s = &s[6..];
            continue;
        }

        break;
    }

    println!("NO");
}
