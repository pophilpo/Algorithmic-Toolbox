fn main() {


    let mut number = String::new();
    ::std::io::stdin().read_line(&mut number).expect("Failed to read line");


    let mut profit = String::new();
    let mut clicks = String::new();

    ::std::io::stdin().read_line(&mut profit).expect("Failed to read line");
    ::std::io::stdin().read_line(&mut clicks).expect("Failed to read line");

    let mut profit = profit.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    let mut clicks = clicks.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i64>>();

    profit.sort();
    clicks.sort();
    profit.reverse();
    clicks.reverse();
    

    let mut result = 0;
    for (prof, click) in profit.iter().zip(clicks) {
        result = result + prof * click;

    }

    println!("{}", result);
}