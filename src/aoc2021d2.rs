pub fn p1(_input: Vec<String>) -> String {
	let input = map_to_instr(_input);

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
	
	(sub.hor_poz * sub.depth).to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	let input = map_to_instr(_input);

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
	
	(sub.hor_poz * sub.depth).to_string()
}

// setup below

enum Instruction {
	Forward(u32),
	Down(u32),
	Up(u32),
	Aim(u32),
}

struct Submarine {
	hor_poz: u32,
	depth: u32,
	aim: Option<u32>,
}

fn map_to_instr(input: Vec<String>) -> Vec<Instruction> {
	input.into_iter().map(|l| parse_instruction(l)).collect()
}

fn parse_instruction(instruction: String) -> Instruction {
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