
fn main() {


    let mut array = String::new();
    ::std::io::stdin().read_line(&mut array).expect("Failed to read line!");
    let mut query = String::new();
    ::std::io::stdin().read_line(&mut query).expect("Failed to read line");
    let mut array = array.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u64>>();
    let mut query = query.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u64>>();

    array.remove(0);
    query.remove(0);


    let mut result: Vec<i64> = Vec::new();


    for key in query.iter(){

        let id = array.binary_search(key);

        match id {
            Ok(id) => result.push(id as i64),
            Err(_) => result.push(-1),
        };


    }

    let output: String = result.iter().map(|x| x.to_string() + " ").collect();
    println!("{}", output);

}