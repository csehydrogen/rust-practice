use std::io::Write;

fn main() {
    // Print query
    print!("n =? "); std::io::stdout().flush().expect("fuck");

    // Read input
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("fuck");
    let n: i32 = n.trim().parse().expect("fuck");

    // Calculate nth fibonacci
    let (mut cnt, mut f0, mut f1) = (0, 0, 1);
    let mut f2;
    while cnt < n {
        f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
        cnt += 1;
    }

    // Print
    println!("f({}) = {}", n, f0);
}
