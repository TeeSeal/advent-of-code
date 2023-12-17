use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut gears = HashMap::new();
    let mut prev_numeric = false;
    let mut part_1 = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, chr) in line.iter().enumerate() {
            if !chr.is_numeric() {
                prev_numeric = false;
                continue;
            }

            if prev_numeric {
                continue;
            }
            prev_numeric = true;

            let str_num = line[x..]
                .iter()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let num = str_num.parse::<i32>().unwrap();

            let last_x = x + str_num.len() - 1;

            let min_x = if x == 0 { 0 } else { x - 1 };
            let max_x = if last_x + 1 >= line.len() {
                last_x
            } else {
                last_x + 1
            };
            let min_y = if y == 0 { 0 } else { y - 1 };
            let max_y = if y + 1 >= grid.len() { y } else { y + 1 };


            let mut is_part = false;

            for j in min_y..=max_y {
                for i in min_x..=max_x {
                    let c = grid[j][i];
                    if !c.is_numeric() && c != '.' {
                        is_part = true;

                        if c == '*' {
                            gears.entry((i, j)).or_insert(Vec::new()).push(num);
                        }
                    }
                }
            }

            if is_part {
                part_1 += num;
            }
        }
    }

    println!("Part 1: {}", part_1);

    let part_2: i32 = gears.values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum();
    println!("Part 2: {}", part_2);

}
