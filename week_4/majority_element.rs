

fn main() {
    
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("Failed to read line");
    let mut input = String::new();
    ::std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u64>>();


    let mut count = 0;

    let mut candidate = 0;
    for value in input.iter() {

        if count == 0 {
            candidate = *value;
            count = count + 1;
        } else {

            if candidate == *value {
                count = count + 1;
            } else {
                count = count - 1;
            }
            
        }


    }

    let mut count = 0;
    let mut result = false;
    for value in input.iter() {
        if *value == candidate {
            count = count + 1;
        } 
    }
    if count > input.len() / 2 {


        result = true;
    }

    match result {
        true => println!("1"),
        false => println!("0"),
    };


}