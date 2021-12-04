fn main() {
	let my_inp = read_lines();
	
	println!("Part 1: {:?}", part_1_solution(&my_inp));
	println!("Part 2: {:?}", part_2_solution(&my_inp));

}

fn read_lines() -> Vec<u32> {
	let input = include_str!("sonar_sweep.txt");
	input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1_solution(lines: &Vec<u32>) -> u32 {

	let mut counter: u32 = 0;
	let mut iter = lines.into_iter().peekable();

	while let Some(next_value) = iter.next() {
		let peek_value = iter.peek();

		if let Some(new_val) = peek_value {
			if new_val > &next_value {
				counter += 1;
			}
		}
	}

	return counter;
}

fn part_2_solution(lines: &Vec<u32>) -> u32 {

	let mut counter: u32 = 0;
	let mut previous_value = None;

	for window in lines.windows(3) {
		let sum = window.iter().sum::<u32>();
		if let Some(prev) = previous_value {
			if prev < sum {
				counter += 1;
			}
		}
		previous_value = Some(sum);
	}

	return counter;
}