struct Bag {
    space_left: u64,
    value: f64,
}

fn main() {
    let mut initial_input = String::new();
    ::std::io::stdin()
        .read_line(&mut initial_input)
        .expect("Failed to read line");
    let paramters = initial_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    let number_of_lines = paramters[0];
    let max_weight = paramters[1];

    let mut data: Vec<(u64, u64)> = Vec::new();
    for _ in 0..number_of_lines {
        let mut line = String::new();
        ::std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let numbers = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u64>>();
        data.push((numbers[0], numbers[1]));
    }
    data.sort_by(|a, b| (a.0 * b.1).cmp(&(b.0 * a.1)));
    data.reverse();

    let mut bag = Bag {
        space_left: max_weight,
        value: 0.0,
    };

    for pair in data.iter() {
        let frac_size: f64 = pair.0 as f64 / pair.1 as f64;
        let mut size = 0;
        if pair.1 >= bag.space_left {
            size = bag.space_left;
        } else {
            size = pair.1;
        }
        let value: f64 = size as f64 * frac_size;
        bag.value = bag.value + value;
        bag.space_left = bag.space_left - size;
        if bag.space_left == 0 {
            break;
        }
    }

    println!("{:.4}", bag.value);
}
