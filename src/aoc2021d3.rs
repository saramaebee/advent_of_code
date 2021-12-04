pub fn p1(measurements: Vec<String>) -> String {
	let mut count_of_1: Vec<u32> = vec![0; measurements[0].len()];
	let total_count: u32 = measurements.clone().into_iter().count().try_into().unwrap();

	for _m in measurements {
		let m: Vec<&str> = _m.split("").skip(1).collect();

		for i in 0..m.len()-1 {
			count_of_1[i] += m[i].parse::<u32>().unwrap();
		}
	}

	let mut _gamma: Vec<String> = count_of_1.clone().into_iter().map(|l| if l > (total_count / 2) { 1 } else { 0 }).map(|l: u32| l.to_string()).collect();
	let mut _epsilon: Vec<String> = count_of_1.into_iter().map(|l| if l < (total_count / 2) { 1 } else { 0 }).map(|l: u32| l.to_string()).collect();
	let gamma: isize = isize::from_str_radix(_gamma.join("").as_str(), 2).unwrap();
	let epsilon: isize = isize::from_str_radix(_epsilon.join("").as_str(), 2).unwrap();

	(gamma * epsilon).to_string()
}

pub fn p2(_measurements: Vec<String>) -> String {
	// oxygen rating
	let mut measurements = _measurements.clone();
	let mut i = 0;

	while measurements.len() > 1 && i < measurements[0].len() {
		let x = measurements.clone().iter().filter(|l| l.chars().nth(i).unwrap() == '1').count();
		measurements = measurements.iter().filter(|l| {
			l.chars().nth(i).unwrap() == if x as f64 >= (measurements.len() as f64 / 2 as f64) { '1' } else { '0' }
		}).cloned().collect();
		i += 1;
	}
	
	let oxygen_rating = isize::from_str_radix(measurements[0].as_str(), 2).unwrap();
	
	// co2 rating
	
	measurements = _measurements.clone();
	i = 0;
	
	while measurements.len() > 1 && i < measurements[0].len() {
		let x = measurements.clone().iter().filter(|l| l.chars().nth(i).unwrap() == '1').count();
		measurements = measurements.iter().filter(|l| {
			l.chars().nth(i).unwrap() == if (x as f64) < (measurements.len() as f64 / 2 as f64) { '1' } else { '0' }
		}).cloned().collect();
		i += 1;
	}
	
	let co2_rating = isize::from_str_radix(measurements[0].as_str(), 2).unwrap();

	(oxygen_rating * co2_rating).to_string()

}