fn main() {
	let my_inp = read_lines();
	count_increases(my_inp);
}

fn read_lines() -> Vec<u32> {
	let input = include_str!("sonar_sweep.txt");

	input.lines().map(|l| l.parse().unwrap()).collect()
}

fn count_increases(lines: Vec<u32>) {

	let mut counter = 0;
	let mut iter = lines.into_iter().peekable();

	while let Some(next_value) = iter.next() {
		let peek_value = iter.peek();

		if let Some(new_val) = peek_value {
			if new_val > &next_value {
				counter += 1;
			}
		}
	}

	println!("{}", counter.to_string());

}