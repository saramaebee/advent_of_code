<<<<<<< HEAD
use std::ops::RangeBounds;

pub fn p1(_input: Vec<String>) -> String {
	let input: Vec<Vec<String>> = _input.into_iter().map(|j| {
		j.split("|").map(|k| k.to_string()).collect::<Vec<String>>()[1]
			.split_whitespace().into_iter().filter(|l| l.len() > 0)
			.map(|m| m.to_string()).collect::<Vec<String>>()
	}).collect();
	
	let mut count = 0; // count of 1 4 7 8

	for line in input {
		for token in line {
			// println!("{}", token);
			match token.len() {
				2 => { count += 1 },
				4 => { count += 1 },
				3 => { count += 1 },
				7 => { count += 1 },
				_ => {}
			}
		}
	}

	count.to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	let input: Vec<Vec<String>> = _input.into_iter().map(|k| k.split_whitespace().into_iter().filter(|k| k.len() > 1).map(|k| k.to_string()).collect()).collect();

	for line in input {

		let mut decoded_letters: Vec<String> = Vec::new();

		let mut one: Vec<&str>  = Vec::new();
		let mut two: Vec<&str>  = Vec::new();
		let mut three: Vec<&str>  = Vec::new();
		let mut four: Vec<&str>  = Vec::new();
		let mut seven: Vec<&str>  = Vec::new();
		let mut eight: Vec<&str>  = Vec::new();

		let mut top_seg: String = "".to_string();
		let mut mid_seg: String = "".to_string();
		let mut bottom_left_seg: String = "".to_string();

		line.iter().for_each(|token| {
			match token.len() {
				2 => {
					if one.len() < 1 {
						let mut sorted = token.split("").filter(|l| l.len() > 0).collect::<Vec<&str>>();
						sorted.sort();
						one = sorted;
					}
				},
				4 => {
					if four.len() < 1 {
						let mut sorted = token.split("").filter(|l| l.len() > 0).collect::<Vec<&str>>();
						sorted.sort();
						four = sorted;
					}
				},
				3 => {
					if seven.len() < 1 {
						let mut sorted = token.split("").filter(|l| l.len() > 0).collect::<Vec<&str>>();
						sorted.sort();
						seven = sorted;
					}
					if one.len() > 0 && top_seg.len() == 0 {
						top_seg = seven.clone().into_iter().filter(|k| if k.len() > 0 { !one.contains(k) } else { false }).collect::<Vec<&str>>()[0].to_string();

					}
				},
				7 => {
					if eight.len() < 1 {
						let mut sorted = token.split("").filter(|l| l.len() > 0).collect::<Vec<&str>>();
						sorted.sort();
						eight = sorted;
					}
				},
				5 => {
					// Checking if is two
					let mut sorted = token.split("").filter(|l| l.len() > 0).collect::<Vec<&str>>();
					sorted.sort();
					if two.len() == 0 {
						if seven.len() > 0 {
							let temp: Vec<String> = sorted.clone().into_iter().filter(|k| !seven.contains(k)).map(|k| k.to_string()).collect();
							// determining segments
							if four.len() > 0 {
								let chars_in_four: Vec<String> = temp.clone().into_iter().filter(|k| four.contains(&(k.as_str()))).map(|k| k.to_string()).collect();
								if chars_in_four.len() == 1 {
									two = sorted.clone();
									mid_seg = chars_in_four[0].clone();
									decoded_letters.push(mid_seg.clone());
								}
							}
						}
					}
					if four.len() > 0 {
						let temp: Vec<String> = sorted.clone().into_iter().filter(|k| !four.contains(k)).map(|k| k.to_string()).collect();
						if temp.len() == 1 {
							bottom_left_seg = temp[0].clone();
						}
					}
				}
				_ => {}
			}
		});

		println!("2: {:?} /// 4: {:?} /// mid_seg: {}", two, four, mid_seg);
		println!("3: {:?} /// 8: {:?} /// bottom_left_seg: {}", three, eight, bottom_left_seg);
		println!("1: {:?} /// 7: {:?} /// top_seg: {}", one, seven, top_seg);
	}
	
	
	"Hi".to_string()
}
=======
pub fn p1(_input: Vec<String>) -> String {
	let input: Vec<String> = _input.into_iter().map(|l| l.split("|").map(|k| k.to_string()).collect()).collect();
	let tokens = input.into_iter();

	"Hello World".to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	"Hello World".to_string()
}



// count the occurences of 1 4 7 8 in the string
// 1: len = 2
// 4: len = 4
// 7: len = 3
// 8: len = 7
>>>>>>> b35578efc3ebb5a77dd9f4a2a2a6ae387ba8cf60
