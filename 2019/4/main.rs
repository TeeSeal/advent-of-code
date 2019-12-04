fn all_ascendent(digits: &Vec<u8>) -> bool {
    for pair in digits.windows(2) {
        if let [a, b] = pair {
            if b < a {
                return false;
            }
        }
    }

    true
}

fn has_two_same_adjacent(digits: &Vec<u8>) -> bool {
    let mut current = digits[0];
    let mut consecutive = 1;

    for (idx, &digit) in digits.iter().skip(1).enumerate() {
        if digit == current {
            consecutive += 1
        };
        if consecutive == 2 && idx == digits.len() - 2 {
            return true;
        }

        if digit != current {
            if consecutive == 2 {
                return true;
            }
            current = digit;
            consecutive = 1;
        }
    }

    false
}

fn is_valid_password(number: &u32) -> bool {
    let digits: Vec<u8> = number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

    digits.len() == 6 && has_two_same_adjacent(&digits) && all_ascendent(&digits)
}

fn main() {
    let password_count = (402328..=864247).into_iter().filter(is_valid_password).count();
    println!("Password count: {}", password_count);
}
