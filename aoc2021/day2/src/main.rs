fn part_1(input: &Vec<Instruction>) -> Submarine {
	let mut sub = Submarine {
		hor_poz: 0,
		depth: 0,
		aim: None,
	};
	
	for instruction in input {
		match instruction {
			Instruction::Forward(amt) => sub.hor_poz += amt,
			Instruction::Down(amt) => sub.depth += amt,
			Instruction::Up(amt) => sub.depth -= amt,
			_ => panic!("unrecognised instruction"),
		}
	}
	
	return sub;
}

fn part_2(input: &Vec<Instruction>) -> Submarine {
	let mut sub = Submarine {
		hor_poz: 0,
		depth: 0,
		aim: Some(0),
	};
	
	for instruction in input {
		if let Some(aim) = sub.aim {
			match instruction {
				Instruction::Forward(amt) => {
					sub.hor_poz += amt;
					sub.depth += amt * aim;
				}
				Instruction::Down(amt) => sub.aim = Some(aim + amt),
				Instruction::Up(amt) => sub.aim = Some(aim - amt),
				_ => panic!("unrecognised instruction"),
			}
		}
	}
	
	return sub;
}

// setup below

fn read_lines() -> Vec<Instruction> {
	let input = include_str!("input.txt");
	
	input.lines().map(|l| parse_instruction(l)).collect()
}

fn parse_instruction(instruction: &str) -> Instruction {
	let t = instruction
	.split_whitespace()
	.take(2)
	.collect::<Vec<&str>>();
	if let [dir, amt] = &t[..] {
		match dir {
			&"forward" => return Instruction::Forward(amt.parse::<u32>().unwrap()),
			&"down" => return Instruction::Down(amt.parse::<u32>().unwrap()),
			&"up" => return Instruction::Up(amt.parse::<u32>().unwrap()),
			&"aim" => return Instruction::Aim(amt.parse::<u32>().unwrap()),
			_ => panic!("invalid instruction"),
		}
	} else {
		panic!("owo what's wrong");
	}
}

struct Submarine {
	hor_poz: u32,
	depth: u32,
	aim: Option<u32>,
}

impl std::fmt::Display for Submarine {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if let Some(_aim) = self.aim {
			write!(
				f,
				"(Horizontal position: {}, Depth: {}, Aim: {})",
				self.hor_poz, self.depth, _aim
			)
		} else {
			write!(
				f,
				"(Horizontal position: {}, Depth: {})",
				self.hor_poz, self.depth
			)
		}
	}
}

enum Instruction {
	Forward(u32),
	Down(u32),
	Up(u32),
	Aim(u32),
}

fn main() {
	let my_inp = read_lines();

	let part_1_sub: Submarine = part_1(&my_inp);
	let part_2_sub: Submarine = part_2(&my_inp);

	println!("Part 1 deets: {}", part_1_sub);
	println!("Part 1 answer: {}", part_1_sub.hor_poz * part_1_sub.depth);

	println!("Part 2 deets: {}", part_2_sub);
	println!("Part 2 answer: {}", part_2_sub.hor_poz * part_2_sub.depth);
}
