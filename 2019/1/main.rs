use std::fs::read_to_string;

const INPUT_PATH: &str = "./input.txt";

fn find_fuel_required(mass: i32) -> i32 {
    let fuel_required = mass / 3 - 2;
    if fuel_required <= 0 { return 0 }
    fuel_required + find_fuel_required(fuel_required)
}

fn main() {
    let fuel_required = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .lines()
        .map(|line| {
            let mass: i32 = line.parse().expect("couldn't parse a line");
            find_fuel_required(mass)
        })
        .fold(0, |total, fuel| total + fuel);

    println!("Fuel required = {}", fuel_required);
}
