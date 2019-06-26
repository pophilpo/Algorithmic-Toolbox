fn get_fib_number(id: u64) -> u64 {
    let id = id % 60;
    if id <= 1 {
        id
    } else {
        let mut prev = 1;
        let mut current = 1;

        for _ in 2..id {
            let buffer = current;
            current = (prev + current) % 10;
            prev = buffer;
        }
        current
    }
}

fn main() {
    let mut input = String::new();
    ::std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let data: u64 = input.trim().parse().unwrap();

    let first = get_fib_number(data);
    let second = get_fib_number(data + 1);
    println!("{}", (first * second) % 10);
}
