fn main() {
    let mut input = String::new();
    ::std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    numbers.sort();

    let mut a = numbers[0];
    let mut b = numbers[1];
    while a > 0 && b > 0 {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    println!("{}", a + b);
}
