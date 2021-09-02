use proconio::input;

fn main() {
    input! {
        mut n: String,
        k: usize,
    }

    for _ in 0..k {
        n = to_base9(&n).replace("8", "5");
    }

    println!("{}", n);
}

fn to_base9(n: &String) -> String {
    let mut v = n.chars().rev().enumerate().fold(0, |acc, (i, v)| {
        acc + v.to_digit(8).unwrap() as usize * 8_usize.pow(i as _)
    });
    dbg!(v);

    let mut ans = String::new();
    loop {
        ans.insert_str(0, &(v % 9).to_string());
        v /= 9;
        if v == 0 {
            break;
        }
    }

    ans
}
