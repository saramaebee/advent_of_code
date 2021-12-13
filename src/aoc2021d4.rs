pub fn p1(_input: Vec<String>) -> String {
	let nums_called: Vec<isize> = _input[0]
		.split(",")
		.map(|l| l.parse::<isize>().unwrap())
		.collect();

	let mut playing_boards = extract_boards(_input);

	for num_called in nums_called.iter().peekable() {
		playing_boards = run_number(playing_boards, *num_called);

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
	let nums_called: Vec<isize> = _input[0]
	.split(",")
	.map(|l| l.parse::<isize>().unwrap())
	.collect();

	let mut playing_boards: Vec<Vec<Vec<isize>>> = extract_boards(_input);

	for num_called in nums_called {
		playing_boards = run_number(playing_boards, num_called);

		if playing_boards.len() > 1 {
			playing_boards.retain(|l| !check_board(l));
		} else {
			let sum = sum_of_board(&playing_boards[0]);
			return (sum * num_called).to_string();
		}
	}

	"No solution found".to_string()
}

// setup funcs

fn run_number(playing_boards: Vec<Vec<Vec<isize>>>, num_called: isize) -> Vec<Vec<Vec<isize>>> {
	playing_boards.into_iter().map(|l| {
		l.clone().iter().map(|n| {
			n.clone().iter().map(|p| if *p == num_called { -1 } else { *p }).collect()
		}).collect()
	}).collect::<Vec<Vec<Vec<isize>>>>()
}

fn check_board(_board: &Vec<Vec<isize>>) -> bool {
	for row in _board {
		if row.iter().all(|l| *l == -1) {
			return true;
		}
	}
	
	for i in 0.._board.len() {

		let column = &_board.clone().into_iter().map(|l| l[i]).collect::<Vec<isize>>();
		if column.iter().all(|l| *l == -1) {
			return true;
		}
	}

	false
}

fn sum_of_board(vector: &Vec<Vec<isize>>)  -> isize {

	let board: Vec<Vec<isize>> = vector.clone().into_iter().map(|k| {
		let m = k.into_iter().filter(|l| *l >= 0).map(|l| l).collect::<Vec<isize>>();
		m.to_vec()
	}).collect();

	let mut sum = 0;

	for row in board {
		sum += row.iter().sum::<isize>();
	}

	sum
}

fn extract_boards(_input: Vec<String>) -> Vec<Vec<Vec<isize>>>{
	let input: Vec<String> = _input
		.into_iter()
		.filter(|l| l.len() > 0 && l.len() < 40)
		.collect();

	let boards: Vec<Vec<Vec<isize>>> = input
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
								n.parse::<isize>().unwrap()
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