mod aoc2021d1;
mod aoc2021d2;
mod aoc2021d3;

fn main() {
	println!("Advent of Code");
	println!("| Day 1");
	println!("| | Part 1: {}", aoc2021d1::p1(parse_input("aoc2021d1.txt")));
	println!("| | Part 2: {}", aoc2021d1::p2(parse_input("aoc2021d1.txt")));
	println!("| Day 2");
	println!("| | Part 1: {}", aoc2021d2::p1(parse_input("aoc2021d2.txt")));
	println!("| | Part 2: {}", aoc2021d2::p2(parse_input("aoc2021d2.txt")));
	println!("| Day 3");
	println!("| | Part 1: {}", aoc2021d3::p1(parse_input("aoc2021d3.txt")));
	println!("| | Part 2: {}", aoc2021d3::p2(parse_input("aoc2021d3.txt")));
	
}

fn parse_input(input_path: &str) -> Vec<String> {
	let input = std::fs::read_to_string(&("src/".to_owned() + input_path)).unwrap();
	input.lines().map(|l| l.parse::<String>().unwrap()).collect()
}

//  fn demo_func(input: Vec<String>) -> String {
//  	this is an example of a function
//  }