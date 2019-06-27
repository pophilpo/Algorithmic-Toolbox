struct Car {
    distance_traveled: u64,
    fuel_left: u64,
    number_of_refils: u64,
}

fn main() {
    let mut distance = String::new();
    ::std::io::stdin()
        .read_line(&mut distance)
        .expect("Failed to read line!");
    let distance: u64 = distance.trim().parse().expect("This is not a number!");
    let mut tank_capacity = String::new();
    ::std::io::stdin()
        .read_line(&mut tank_capacity)
        .expect("Failed to read line!");
    let tank_capacity: u64 = tank_capacity.trim().parse().expect("Failed to read line!");
    let mut number_of_stops = String::new();
    ::std::io::stdin()
        .read_line(&mut number_of_stops)
        .expect("Failed to read line!");
    let mut stops = String::new();
    ::std::io::stdin()
        .read_line(&mut stops)
        .expect("Failed to read line!");
    let mut stops = stops
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut car = Car {
        distance_traveled: 0,
        fuel_left: tank_capacity,
        number_of_refils: 0,
    };

    stops.push(distance);

    for stop in stops {
        if car.distance_traveled + tank_capacity < stop {
            println!("-1");
            ::std::process::exit(1);
        } else {
            if car.distance_traveled + car.fuel_left < stop {
                car.number_of_refils = car.number_of_refils + 1;
                car.fuel_left = tank_capacity;
            }
            car.fuel_left = car.fuel_left - (stop - car.distance_traveled);
            car.distance_traveled = stop;
        }
    }
    println!("{}", car.number_of_refils);
}
