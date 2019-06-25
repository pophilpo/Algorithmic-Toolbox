struct Sec {
    prev: u64,
    current: u64,
}

fn main() {
    let mut id = String::new();
    ::std::io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");
    let id = id.trim().parse::<u64>().expect("This is not a number");

    if id <= 1 {
        println!("{}", id);
    } else {
        let mut fib_numbers = Sec {
            prev: 1,
            current: 1,
        };

        for _ in 2..id {
            let buffer = fib_numbers.current;
            fib_numbers.current = fib_numbers.prev + fib_numbers.current;
            fib_numbers.prev = buffer;
        }
        println!("{}", fib_numbers.current);
    }
}
