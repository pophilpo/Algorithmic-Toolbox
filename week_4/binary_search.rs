
enum Index {
    Some(usize),
    None(),
}


fn binary_search(data: &Vec<u64>, left: usize, right: usize, key: u64) -> Index {


    if left >= right {
        return Index::None()
    } else {

        let mid = left + (right-left) / 2;

        if data[mid] == key {
            return Index::Some(mid);
        } else if data[mid] > key {
            return binary_search(data, left, mid, key);
        } else {
            return binary_search(data, mid, right, key);
        }

    }

}


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

        let id = binary_search(&array, 0, array.len(), *key);

        match id {
            Index::Some(i) => result.push(i as i64),
            Index::None() => result.push(-1),
        };

    }

    let output: String = result.iter().map(|x| x.to_string() + " ").collect();
    println!("{}", output);

}