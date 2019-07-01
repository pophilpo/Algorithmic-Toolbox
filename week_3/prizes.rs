fn main() {
    let mut input = String::new();
    ::std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let mut input: i64 = input.trim().parse().expect("This is not a number");
    let mut result: Vec<i64> = Vec::new();

    if input <= 2 {
        println!("1");
        println!("{}", input);
    } else {
        for candidate in 1..input {
            let dif = input - candidate;
            if result.contains(&dif) || dif == candidate {
                result.push(input);
                break;
            } else {
                input = dif;
                result.push(candidate);
            }
            if input <= 0 {
                break;
            }
        }

        println!("{}", result.len());
        let output: String = result.iter().map(|x| x.to_string() + " ").collect();
        println!("{}", output);
    }
}
