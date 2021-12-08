use std::ops::Range;

pub fn p1(_input: Vec<String>) -> String {
	let input: Vec<isize> = _input[0].split(",").map(|l| l.parse::<isize>().unwrap()).collect();

	let min: isize = *input.clone().iter().min().unwrap();
	let max: isize = *input.clone().iter().max().unwrap();

	let mut min_sum: isize = 9999999;

	for i in min .. max + 1 {
		let sum = input.clone().into_iter().map(|l| (i - l).abs()).sum();
		if sum < min_sum {
			min_sum = sum;
		}
	}

	min_sum.to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	let input: Vec<isize> = _input[0].split(",").map(|l| l.parse::<isize>().unwrap()).collect();

	let min: isize = *input.clone().iter().min().unwrap();
	let max: isize = *input.clone().iter().max().unwrap();

	let mut min_sum: isize = 99999999999;

	for i in min .. max + 1 {
		let sum = input.clone().into_iter().map(|l| {
			sum_of_range(0 .. (i - l).abs() + 1)
		}).sum();
		if sum < min_sum {
			min_sum = sum;
		}
	}

	min_sum.to_string()
}

// helper funcs
fn sum_of_range( range: Range<isize>) -> isize {
	range.fold(0, |a, b| a + b)
}