pub fn p1(_input: Vec<String>) -> String {
	let nums_called: Vec<i32> = _input[0]
		.split(",")
		.map(|l| l.parse::<i32>().unwrap())
		.collect();

	let mut playing_boards = extract_boards(_input);

	for num_called in nums_called.iter().peekable() {
		playing_boards = playing_boards.into_iter().map(|l| {
			let k = l.clone().iter().map(|n| {
				let o = n.clone().iter().map(|p| if *p == *num_called { -1 } else { *p }).collect();
				o
			}).collect();
			k
		}).collect();

		for board in &playing_boards {
			if check_board(board) {
				let sum = sum_of_board(board);
				return (sum * num_called).to_string();
			}
		}
	}

	"No solution found".to_string()
}

// My part 2 solution isn't finished. It works on the example input provided in the problem,
// but it's not working correctly on the actual text for some reason.

pub fn p2(_input: Vec<String>) -> String {
	let nums_called: Vec<i32> = _input[0]
	.split(",")
	.map(|l| l.parse::<i32>().unwrap())
	.collect();

	let mut playing_boards: Vec<Vec<Vec<i32>>> = extract_boards(_input);

	for num_called in nums_called.iter().peekable() {
		playing_boards = playing_boards.into_iter().map(|l| {
			let k = l.clone().iter().map(|n| {
				let o = n.clone().iter().map(|p| if *p == *num_called { -1 } else { *p }).collect();
				o
			}).collect();
			k
		}).collect::<Vec<Vec<Vec<i32>>>>();

		if playing_boards.len() > 1 {
			playing_boards.retain(|l| !check_board(l));
		} else {
			let sum = sum_of_board(&playing_boards[0]);
			// println!("Sum: {:?}; Last Num: {:?}; Winner: {:?}", sum, num_called, playing_boards);
			return (sum * num_called).to_string();
		}
	}

	"No solution found".to_string()
}

// setup funcs

fn check_board(_board: &Vec<Vec<i32>>) -> bool {
	
	for row in _board {
		if row.iter().all(|l| *l == -1) {
			return true;
		}
	}
	
	for i in 0.._board.len() {

		let column = &_board.clone().into_iter().map(|l| l[i]).collect::<Vec<i32>>();
		if column.iter().all(|l| *l == -1) {
			return true;
		}
	}

	false
}

fn sum_of_board(vector: &Vec<Vec<i32>>)  -> i32 {

	let board: Vec<Vec<i32>> = vector.clone().into_iter().map(|k| {
		let m = k.into_iter().filter(|l| *l >= 0).map(|l| l).collect::<Vec<i32>>();
		m.to_vec()
	}).collect();

	let mut sum = 0;

	for row in board {
		sum += row.iter().sum::<i32>();
	}

	sum
}

fn extract_boards(_input: Vec<String>) -> Vec<Vec<Vec<i32>>>{
	let input: Vec<String> = _input
		.into_iter()
		.filter(|l| l.len() > 0 && l.len() < 40)
		.collect();

	let boards: Vec<Vec<Vec<i32>>> = input
		.chunks(5)
		.collect::<Vec<&[String]>>()
		.into_iter()
		.map(|l| {
			l.into_iter()
				.map(|m| {
					m.split(" ")
						.into_iter()
						.filter(|x| x.len() > 0)
						.map(|n| {
							if n.len() > 0 {
								n.parse::<i32>().unwrap()
							} else {
								0
							}
						})
						.collect()
				})
				.collect()
		})
		.collect();

		boards
}