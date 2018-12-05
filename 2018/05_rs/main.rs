use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc;

fn opposite_polarity(c1: char, c2: char) -> bool {
	return (c1.is_uppercase() && c2.is_lowercase()) || (c1.is_lowercase() && c2.is_uppercase());
}

fn same_unit(c1: char, c2: char) -> bool {
	return c1.to_lowercase().to_string() == c2.to_lowercase().to_string();
}

fn react(polymer: &Vec<char>) -> usize {
	let mut units = polymer.to_vec();
	let mut i = 0;

	while i < units.len() - 1 {
		let unit = units[i];
		let next = units[i + 1];

		if same_unit(unit, next) && opposite_polarity(unit, next) {
			units.remove(i);
			units.remove(i); // The elements are pushed back on first remove
			if i > 0 { i -= 1; }
		} else {
			i += 1;
		}
	}

	return units.len()
}

fn main() {
	let mut f = File::open("input.txt").expect("input not found");
	let mut input = String::new();
	f.read_to_string(&mut input).expect("coudln't read input");
	input.pop(); // remove trailing newline

	let polymer: Vec<char> = input.chars().collect();
	let mut size = react(&polymer);
	println!("Part 1: {}", size);


	let (tx, rx) = mpsc::channel();

	for chr in b'a'..=b'z' {
		let tx = tx.clone();
		let mut polymer = polymer.to_vec();

		thread::spawn(move || {
				let bad_unit = chr as char;
				polymer.retain(|&c| !same_unit(c, bad_unit));
				tx.send(react(&polymer)).unwrap();
		});
	}

	for _ in 0..26 {
		let adjusted_size = rx.recv().unwrap();
		if adjusted_size < size { size = adjusted_size }
	}

	println!("Part 2: {}", size);
}
