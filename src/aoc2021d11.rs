
pub fn p1(_input: Vec<String>) -> String {
	let mut input: Vec<Vec<usize>> = _input.into_iter().map(|l| l.split("").into_iter().filter(|k| k.len() > 0).map(|l| l.parse::<usize>().unwrap()).collect()).collect();
	let mut flashes: usize = 0;

	for _i in 0 .. 100 {
		// println!("{:?}", input);
		for y in 0 .. input.len() {
			for x in 0 .. input[y].len() {
				flashes += iterate_cell(x, y, &mut input);
			}
		}
	}

	flashes.to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	"Hi".to_string()
}

fn iterate_cell(x: usize, y: usize, board: &mut Vec<Vec<usize>>) -> usize {
	let mut flashes: usize = 0;
	board[y][x] += 1;
	if board[y][x] > 9 {
		flashes += 1;
		board[y][x] = 0;
		let top = y > 0;
		let bottom = y < board.len() - 1;
		let left = x > 0;
		let right = x < board[y].len() - 1;

		if top {
			if left {
				flashes += iterate_cell(x - 1, y - 1, board);
			}
			if right {
				flashes += iterate_cell(x + 1, y - 1, board);
			}
			flashes += iterate_cell(x, y - 1, board);
		}
		if bottom {
			if left {
				flashes += iterate_cell(x - 1, y + 1, board);
			}
			if right {
				flashes += iterate_cell(x + 1, y + 1, board);
			}
			flashes += iterate_cell(x, y + 1, board);
		}

		if left {
			flashes += iterate_cell(x - 1, y, board);
		}

		if right {
			flashes += iterate_cell(x + 1, y, board);
		}

	}

	flashes
}