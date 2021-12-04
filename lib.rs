pub fn parse_input(input_path: &str) {
	let input = std::fs::read_to_string(input_path).unwrap();
	input.lines().map(|l| l.parse().unwrap()).collect()
}