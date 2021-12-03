fn part_1(measurements: Vec<String>) {
	let mut count_of_1: Vec<u32> = vec![0; measurements[0].len()];
	let total_count: u32 = measurements.clone().into_iter().count().try_into().unwrap();

	for _m in measurements {
		let m: Vec<&str> = _m.split("").skip(1).collect();
		// println!("{:?}", _m);
		// println!("{:?}", m);

		for i in 0..m.len()-1 {
			// println!("{}", m[i]);
			count_of_1[i] += m[i].parse::<u32>().unwrap();
		}
	}

	let mut _gamma: Vec<String> = count_of_1.clone().into_iter().map(|l| if l > (total_count / 2) { 1 } else { 0 }).map(|l: u32| l.to_string()).collect();
	let mut _epsilon: Vec<String> = count_of_1.into_iter().map(|l| if l < (total_count / 2) { 1 } else { 0 }).map(|l: u32| l.to_string()).collect();
	let gamma: isize = isize::from_str_radix(_gamma.join("").as_str(), 2).unwrap();
	let epsilon: isize = isize::from_str_radix(_epsilon.join("").as_str(), 2).unwrap();

	println!("{}", gamma);
	println!("{}", epsilon);
	println!("Power Consumption: {}", gamma * epsilon)
	
}

// setup below

fn main() {
	let my_inp = read_lines();

	part_1(my_inp.clone());
}

fn read_lines() -> Vec<String> {
	let input = include_str!("input.txt");
	
	input.lines().map(|l| l.parse::<String>().unwrap()).collect()
}