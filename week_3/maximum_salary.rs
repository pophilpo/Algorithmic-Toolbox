fn get_max(max: &u64, candidate: &u64) -> u64 {

    if *max == 0 {
        *candidate
    } else {

        let mut f = Vec::new();
        let mut l = Vec::new();
        f.push(max);
        f.push(candidate);
        l.push(candidate);
        l.push(max);


        let f: String = f.iter().map(|x| x.to_string()).collect();
        let l: String = l.iter().map(|x| x.to_string()).collect();

        let f: u64 = f.parse().expect("this is not a number!");
        let l: u64 = l.parse().expect("this is not a number!");

        if f > l {
            *max
        } else {
            *candidate
        }
    }

}

fn main() {
    let mut number = String::new();
    ::std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");
    let mut input = String::new();
    ::std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let mut data: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("This is not a number"))
        .collect::<Vec<u64>>();
    


    let mut result: Vec<u64> = Vec::new();

    while &data.len() > &0 {

    let mut max = 0;
    for number in &data {

        max = get_max(&max, &number);
    }
    result.push(max);
    let index = data.iter().position(|x| *x==max).unwrap();
    data.remove(index);

    }


    let output: String = result.iter().map(|x| x.to_string()).collect();
    println!("{}", output);
}
