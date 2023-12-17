use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut result: i32 = 0;
    let words = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in input.lines() {
        let mut first = 0;
        let mut first_index = 999;
        let mut last = 0;
        let mut last_index = 0;

        if let Some(index) = line.find(char::is_numeric) {
            if index <= first_index {
                first_index = index;
                first = line.chars().nth(index).unwrap().to_digit(10).unwrap();
            }
        }

        if let Some(index) = line.rfind(char::is_numeric) {
            if index >= last_index {
                last_index = index;
                last = line.chars().nth(index).unwrap().to_digit(10).unwrap();
            }
        }

        for (word, value) in &words {
            if let Some(index) = line.find(word) {
                if index <= first_index {
                    first_index = index;
                    first = *value;
                }
            }

            if let Some(index) = line.rfind(word) {
                if index >= last_index {
                    last_index = index;
                    last = *value;
                }
            }
        }

        let num: i32 = format!("{}{}", first, last).parse().unwrap();

        result += num;
    }

    println!("Result: {}", result);
}
