pub fn p1(_input: Vec<String>) -> String {
	let input: Vec<Line> = _input.into_iter().map(|l| parse_line(l))
							.filter(|k| (k.p1.x == k.p2.x || k.p1.y == k.p2.y))
							.collect();

	let mut points: Vec<Coordinate> = Vec::new();

	for line in input {
		if is_horizontal(&line){
			let y = line.p1.y;
			for i in line.p1.x .. line.p2.x {
				points.push(Coordinate{ x: i, y });
			}
		}

		if is_vertical(&line) {
			let x = line.p1.x;
			for i in line.p1.y .. line.p2.y {
				points.push(Coordinate { x, y: i})
			}
		}
	}

	let mut count:  Vec<(Coordinate, i32)> = Vec::new();

	for point in &points {
		let filtered: Vec<Coordinate> = points.iter().filter(|l| {
			l.x == point.x && l.y == point.y
		}).map(|m| *m).collect();

		count.push((*point, filtered.len() as i32));
	}

	let count_of_coords = count.iter().filter(|l| l.1 > 1).map(|m| *m).count();

	count_of_coords.to_string()
	
}

pub fn p2(_input: Vec<String>) -> String {
	let input: Vec<Line> = _input.into_iter().map(|l| parse_line(l)).collect();

	"No solution found".to_string()
}

// helper functions

fn is_horizontal(_line: &Line) -> bool {
	_line.p1.y == _line.p2.y
}

fn is_vertical(_line: &Line) -> bool {
	_line.p1.x == _line.p2.x
}

fn parse_line(_line: String) -> Line {
	let line: Vec<String> = _line.split(" -> ").map(|l| l.to_string()).collect();
	let p1_coords: Vec<i32> = line[0].split(",").map(|l| l.parse::<i32>().unwrap()).collect();
	let p2_coords: Vec<i32> = line[1].split(",").map(|l| l.parse::<i32>().unwrap()).collect();

	Line {
		p1: Coordinate { x: p1_coords[0], y: p1_coords[1] },
		p2: Coordinate { x: p2_coords[0], y: p2_coords[1] },
	}
}
#[derive(Debug)]
struct Line {
	p1: Coordinate,
	p2: Coordinate
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
	x: i32,
	y: i32,
}