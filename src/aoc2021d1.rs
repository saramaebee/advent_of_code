pub fn p1(_lines: Vec<String>) -> String {

	let lines = map_to_u32(_lines);

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

	counter.to_string()
}

pub fn p2(_lines: Vec<String>) -> String {

	let lines = map_to_u32(_lines);

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

	counter.to_string()
}

fn map_to_u32(input: Vec<String>) -> Vec<u32> {
	input.into_iter().map(|l| l.parse::<u32>().unwrap()).collect()
}