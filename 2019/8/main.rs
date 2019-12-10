use std::fs::read_to_string;

const INPUT_PATH: &str = "./input.txt";

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn part1(layers: &Vec<Vec<u32>>) -> usize {
    let mut layers = layers.clone();
    layers.sort_by_key(|layer| layer.into_iter().filter(|&&n| n == 0).count());
    let ones = layers[0].iter().filter(|&&n| n == 1).count();
    let zeroes = layers[0].iter().filter(|&&n| n == 2).count();
    ones * zeroes
}

fn part2(layers: &Vec<Vec<u32>>) -> String {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = layers
                .iter()
                .map(|layer| layer[y * WIDTH + x])
                .find(|&n| n != 2)
                .expect("all pixels transparent");

            output.push(if pixel == 0 { ' ' } else { '*' });
        }

        output.push('\n');
    }

    output
}

fn main() {
    let digits: Vec<u32> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let layers: Vec<Vec<u32>> = digits
        .chunks(LAYER_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    println!("Part 1: {}", part1(&layers));
    println!("Part 2: \n{}", part2(&layers));
}
