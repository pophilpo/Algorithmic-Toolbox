fn main() {

    let mut input = String::new();

    ::std::io::stdin().read_line(&mut input).expect("Failed to read line!");
    let data: u64 = input.trim().parse().unwrap();
    let tens: u64 = data / 10;
    let fives: u64 = (data - tens * 10) / 5;
    let ones: u64 =  data - tens *10 - fives * 5;
    let count: u64 = tens + fives + ones;

    println!("{}", count);

}