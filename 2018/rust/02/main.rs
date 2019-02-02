use std::fs::File;
use std::io::prelude::*;

fn different_char_count(s1: &str, s2: &str) -> i32 {
    s1.chars()
        .zip(s2.chars())
        .fold(0, |acc, (x, y)| if x == y { acc } else { acc + 1 })
}

fn main() {
    let mut f = File::open("input.txt").expect("input not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("coudln't read input");
    input.pop(); // remove trailing newline

    let ids: Vec<&str> = input.split_whitespace().collect();

    let mut double_occurances = 0;
    let mut triple_occurances = 0;
    let mut found_pair = false;
    let mut pair1 = "";
    let mut pair2 = "";

    for id in ids.iter() {
        let mut chars: Vec<char> = id.chars().collect();
        chars.sort();
        chars.dedup();

        let char_counts: Vec<usize> = chars.iter()
            .map(|c| id.matches(c.to_string().as_str()).count())
            .collect();

        if char_counts.contains(&2) { double_occurances += 1 }
        if char_counts.contains(&3) { triple_occurances += 1 }

        if found_pair { continue }
        ids.iter()
            .find(|other| different_char_count(id, other) == 1)
            .map(|other| {
                pair1 = id;
                pair2 = other;
                found_pair = true;
            });
    }

    println!("Part 1: {}", double_occurances * triple_occurances);

    let chars1: Vec<char> = pair1.chars().collect();
    let chars2: Vec<char> = pair2.chars().collect();
    let common: String = chars1.iter().filter(|c| chars2.contains(&c)).collect();
    println!("{}", common);
}
