use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort_by(|x, y| x.cmp(y).reverse());

    let (alice, bob) = a.iter().enumerate().fold((0, 0), |(a_acc, b_acc), (i, x)| {
        if i % 2 == 0 {
            (a_acc + x, b_acc)
        } else {
            (a_acc, b_acc + x)
        }
    });

    println!("{}", alice - bob);
}
