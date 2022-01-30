use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Redirect {
	pub source: String,
	pub dest: String
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

pub fn read_csv(file: String) -> Vec<Redirect> {

	if let Ok(lines) = read_lines(file) {
		for line in lines {
			if let Ok(rule) = line {
				println!("{}", rule);
			}
		}
	}

	return vec![];
}