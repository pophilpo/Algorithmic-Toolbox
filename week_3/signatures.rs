fn main() {

    let mut number_of_lines = String::new();
    ::std::io::stdin().read_line(&mut number_of_lines).expect("Failed to read line");
    let number_of_lines: u32 = number_of_lines.trim().parse().expect("This is not a number");

    let mut data: Vec<(u64, u64)> = Vec::new();

    for _ in 0..number_of_lines {

        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).expect("Failed to read line");
        let line = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u64>>();

        data.push((line[0], line[1]));
    }

    data.sort_by(|x, y| x.1.cmp(&y.1));


    let mut result: Vec<u64> = Vec::new();

    let mut endpoint = data[0].1;
    result.push(endpoint);
    for segment in data {


        if segment.0 <= endpoint && endpoint <= segment.1 {
            continue;
        } else {
            endpoint = segment.1;
            result.push(endpoint);
        }

    }


    println!("{}", result.len());
    let output: String = result.iter().map(|x| x.to_string() + " ").collect();
    println!("{}", output);
}