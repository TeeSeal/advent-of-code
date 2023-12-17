type Point = (u64, u64);

fn galaxy_coordinates(image: &str, wormhole_size: usize) -> Vec<Point> {
    let matrix: Vec<Vec<char>> = image.lines().map(|line| line.chars().collect()).collect();

    let mut y_wormholes = Vec::new();
    for (y, line) in matrix.iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            y_wormholes.push(y);
        }
    }

    let mut x_wormholes = Vec::new();
    for x in 0..matrix[0].len() {
        if matrix.iter().all(|line| line[x] == '.') {
            x_wormholes.push(x);
        }
    }

    let mut result = Vec::new();

    for (y, line) in matrix.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != '#' {
                continue;
            }

            let adjusted_x =
                x + x_wormholes.iter().filter(|&&other| other < x).count() * (wormhole_size - 1);
            let adjusted_y =
                y + y_wormholes.iter().filter(|&&other| other < y).count() * (wormhole_size - 1);

            result.push((adjusted_x as u64, adjusted_y as u64))
        }
    }

    result
}

fn manhattan_distance(p1: &Point, p2: &Point) -> u64 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn sum_of_galaxy_distances(image: &str, wormhole_size: usize) -> u64 {
    let mut result = 0;
    let coordinates = galaxy_coordinates(image, wormhole_size);

    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            result += manhattan_distance(&coordinates[i], &coordinates[j]);
        }
    }

    result
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let part_1 = sum_of_galaxy_distances(&input, 2);
    let part_2 = sum_of_galaxy_distances(&input, 1_000_000);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
