mod aoc2021d1;
mod aoc2021d2;
mod aoc2021d3;
mod aoc2021d4;
mod aoc2021d5;

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
		5 aoc2021d5 "aoc2021d5.txt"
	);
}

fn parse_input(input_path: &str) -> Vec<String> {
	let input = std::fs::read_to_string(&("src/".to_owned() + input_path)).unwrap();
	input.lines().map(|l| l.parse::<String>().unwrap()).collect()
}

//  fn demo_func(input: Vec<String>) -> String {
//  	this is an example of a function
//  }