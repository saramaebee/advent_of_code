pub fn p1 (_input: Vec<String>) -> String {
	let mut input: Vec<i32> = _input[0].split(",").map(|l| l.parse::<i32>().unwrap()).collect();

	for _i in 0..80 {
		input = increment_day(input, _i);
	}

	input.len().to_string()
}

pub fn p2 (_input: Vec<String>) -> String {
	let mut input: Vec<i32> = _input[0].split(",").map(|l| l.parse::<i32>().unwrap()).collect();

	for _i in 0..256 {
		input = increment_day(input, _i);
	}

	input.len().to_string()
}

fn increment_day(_fish: Vec<i32>, day: i32) -> Vec<i32> {
	let mut return_vec = Vec::new();
	let mut count_of_fish_born = 0;
	return_vec = _fish.into_iter().map(|l| {
		if &l < &1 {
			count_of_fish_born += 1;
			6
		} else {
			l - 1
		}
	}).collect();

	println!("On day {} there were {} fish born. return_vec.len(): {}", day, count_of_fish_born, return_vec.len());

	let mut a_vec: Vec<i32> = vec![8; count_of_fish_born];

	return_vec.append(&mut a_vec);

	return_vec
}