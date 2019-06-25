struct Sec {
    prev: u32,
    current: u32,
}


fn main() {

    let mut id = String::new();
    ::std::io::stdin().read_line(&mut id).expect("Failed to read line");
    let id: u32 = id.trim().parse().expect("This is not a number");

    if id <= 1 {
        println!("{}", id);
    } else {
        let mut fibonacci = Sec {
            prev: 1,
            current: 1,
        };
        for _ in 2..id {

            let buffer = fibonacci.current;

            fibonacci.current = (fibonacci.prev + fibonacci.current) % 10;
            fibonacci.prev = buffer;
            
        }
        println!("{}", fibonacci.current);
    }


}

