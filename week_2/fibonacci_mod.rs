struct Pisano {
    prev: u64,
    current: u64,
}

struct Sec {
    prev: u64,
    current: u64,
}

fn get_fibonacci_number(id: u64, divisor: u64) -> u64 {
    if id <= 1 {
        id
    } else {
        let mut fib = Sec {
            prev: 1,
            current: 1,
        };
        for _ in 2..id {
            let buffer = fib.current;
            fib.current = (fib.prev + fib.current) % divisor;
            fib.prev = buffer;
        }
        fib.current
    }
}

fn get_pisano(divisor: u64) -> u64 {
    let mut remainders: Vec<u64> = Vec::new();
    let mut checker = Pisano {
        prev: 0,
        current: 1,
    };
    remainders.push(0);
    remainders.push(1);

    for id in 2..divisor * 6 {
        let remainder = get_fibonacci_number(id, divisor);
        remainders.push(remainder);

        checker.prev = checker.current;
        checker.current = remainder;

        if checker.prev == 0 && checker.current == 1 {
            break;
        }
    }
    remainders.len() as u64 - 2
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
    let id = data[0];
    let divisor = data[1];

    let pisano = get_pisano(divisor);
    let final_id = id % pisano;

    let result = get_fibonacci_number(final_id, divisor);
    println!("{}", result);
}
