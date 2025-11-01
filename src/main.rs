use zk_exam::ring::extended_euclidean;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let x: i64 = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let y: i64 = buf.trim().parse().unwrap();
    match extended_euclidean(x, y) {
        Ok((a, b, view)) => {
            println!("{x}*{a} + {y}*{b} = 1");
            println!("{view:?}")
        }
        Err(gcd) => {
            println!("Gcd is {gcd}")
        }
    }
    println!("Hello, world!");
}
