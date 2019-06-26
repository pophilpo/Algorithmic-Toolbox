fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut id = input.trim().parse::<u64>().expect("This is not a number!");

    if id <= 1 {
        println!("{}", id);
    } else {
        id = id + 2;

        let pisano = 60;

        let id = id % pisano;

        let mut prev = 1;
        let mut current = 1;

        for _ in 2..id {
            let buffer = current;
            current = (prev + current) % 10;
            prev = buffer;
        }

        if current == 0 {
            current = 9;
        } else {
            current = current -1;
        }

        println!("{}", current);
    }
}
