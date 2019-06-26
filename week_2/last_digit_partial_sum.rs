fn get_fib_number(id: u64) -> u64 {

    let  id = id % 60;
    if id <= 1 {
        id
    } else {
        let mut prev = 1;
        let mut current = 1;

        for _ in 2..id {
            let buffer = current;
            current = (current + prev) % 10;
            prev = buffer
        }

        current
    }
}

fn main() {
    let mut input = String::new();

    ::std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let data = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut start = data[0];
    let end = data[1];

    if start == end {
        println!("{}", get_fib_number(start));
    } else {
        start = start - 1;
        let full_sum_id = end + 2;
        let part_sum_id = start + 2;

        let mut full_sum = get_fib_number(full_sum_id % 60);
        let mut part_sum = get_fib_number(part_sum_id % 60);

        if full_sum == 0 {
            full_sum = 9;
        } else {
            full_sum = full_sum - 1;
        }

        if part_sum == 0 {
            part_sum = 9;
        } else {
            part_sum = part_sum - 1;
        }

        if full_sum < part_sum {
            full_sum = full_sum + 10;
        }

        println!("{}", full_sum - part_sum);
    }
}
