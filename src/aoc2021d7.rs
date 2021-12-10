use std::ops::Range;

pub fn p1(_input: Vec<String>) -> String {
	let input: Vec<usize> = _input[0].split(",").map(|l| l.parse::<usize>().unwrap()).collect();

	let min: usize = *input.clone().iter().min().unwrap();
	let max: usize = *input.clone().iter().max().unwrap();

	let mut min_sum: usize = input.clone().into_iter().map(|l| (min as isize - l as isize).abs() as usize).sum();

	for i in min .. max + 1 {
		let sum = input.clone().into_iter().map(|l| (i as isize - l as isize).abs() as usize).sum();
		if sum < min_sum {
			min_sum = sum;
		}
	}

	min_sum.to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	let input: Vec<usize> = _input[0].split(",").map(|l| l.parse::<usize>().unwrap()).collect();

	let min: isize = *input.clone().iter().min().unwrap() as isize;
	let max: isize = *input.clone().iter().max().unwrap() as isize;

	let mut min_sum: usize = input.clone().into_iter().map(|l| {
		sum_of_range(0 as usize .. ((min - l as isize).abs() as usize) + 1)
	}).sum();

	for i in min .. max + 1 {
		let sum = input.clone().into_iter().map(|l| {
			sum_of_range(0 as usize .. ((i - l as isize).abs() as usize) + 1)
		}).sum();
		min_sum = sum;
		if sum < min_sum {
			min_sum = sum;
		}
	}

	min_sum.to_string()
}

// helper funcs
fn sum_of_range( range: Range<usize>) -> usize {
	range.fold(0, |a, b| a + b)
}