fn gcd(x: u64, y: u64) -> u64 {
    let mut a = x;
    let mut b = y;

    while a != 0 && b != 0 {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    a + b
}

fn main() {
    let mut input = String::new();

    ::std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let data = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let a = data[0];
    let b = data[1];
    let gcd = gcd(a, b);

    let lcm = (a * b) / gcd;
    println!("{}", lcm);
}
