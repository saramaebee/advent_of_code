#![feature(map_try_insert)]

mod aoc2021d1;
mod aoc2021d2;
mod aoc2021d3;
mod aoc2021d4;
mod aoc2021d5;
mod aoc2021d6;
mod aoc2021d7;
mod aoc2021d8;
mod aoc2021d9;
mod aoc2021d10;
mod aoc2021d11;

macro_rules! run {
	($($day_num:tt $module:tt $file_name:tt),*) => (
		$(
			println!("| Day {}", $day_num);
			println!("| | Part 1: {}", $module::p1(parse_input($file_name)));
			println!("| | Part 2: {}", $module::p2(parse_input($file_name)));
		)*
	);
}

fn main() {
	println!("Advent of Code");
	run!(
		1 aoc2021d1 "aoc2021d1.txt", 
		2 aoc2021d2 "aoc2021d2.txt", 
		3 aoc2021d3 "aoc2021d3.txt",
		4 aoc2021d4 "aoc2021d4.txt",
		5 aoc2021d5 "aoc2021d5.txt",
		6 aoc2021d6 "aoc2021d6.txt",
		// 7 aoc2021d7 "aoc2021d7.txt",
		// 8 aoc2021d8 "aoc2021d8.txt",
		9 aoc2021d9 "aoc2021d9.txt",
		10 aoc2021d10 "aoc2021d10.txt",
		11 aoc2021d11 "aoc2021d11.txt"
		// 12 aoc2021d12 "aoc2021d12.txt", 
		// 13 aoc2021d13 "aoc2021d13.txt",
		// 14 aoc2021d14 "aoc2021d14.txt",
		// 15 aoc2021d15 "aoc2021d15.txt",
		// 16 aoc2021d16 "aoc2021d16.txt",
		// 17 aoc2021d17 "aoc2021d17.txt",
		// 18 aoc2021d18 "aoc2021d18.txt",
		// 19 aoc2021d19 "aoc2021d19.txt",
		// 20 aoc2021d20 "aoc2021d20.txt",
		// 21 aoc2021d21 "aoc2021d21.txt",
		// 22 aoc2021d22 "aoc2021d22.txt", 
		// 23 aoc2021d23 "aoc2021d23.txt",
		// 24 aoc2021d24 "aoc2021d24.txt",
		// 25 aoc2021d25 "aoc2021d25.txt",
		// 26 aoc2021d26 "aoc2021d26.txt",
		// 27 aoc2021d27 "aoc2021d27.txt",
		// 28 aoc2021d28 "aoc2021d28.txt",
		// 29 aoc2021d29 "aoc2021d29.txt",
		// 30 aoc2021d30 "aoc2021d30.txt",
		// 31 aoc2021d31 "aoc2021d31.txt"
	);
}

fn parse_input(input_path: &str) -> Vec<String> {
	let input = std::fs::read_to_string(&("src/".to_owned() + input_path)).unwrap();
	input.lines().map(|l| l.parse::<String>().unwrap()).collect()
}

//  fn demo_func(input: Vec<String>) -> String {
//  	this is an example of a function
//  }