use std::fmt::Debug;

pub fn p1(_input: Vec<String>) -> String {

	let points_register: Vec<Vec<usize>> = _input.into_iter().map(|l| l.split("").filter(|j| j.len() > 0).into_iter().map(|k| k.parse::<usize>().unwrap()).collect()).collect();
	let mut sum: usize = 0;

	for y in 0 .. points_register.len() {
		for x in 0 .. points_register[0].len() {
			if is_low_point(x, y, &points_register) {
				sum += points_register[y][x] + 1;
			}
		}
	}

	sum.to_string()
}

pub fn p2(_input: Vec<String>) -> String {

	let points_register: Vec<Vec<usize>> = _input.into_iter().map(|l| l.split("").filter(|j| j.len() > 0).into_iter().map(|k| k.parse::<usize>().unwrap()).collect()).collect();
	let mut basins: Vec<(usize, usize)> = Vec::new();

	for y in 0 .. points_register.len() {
		for x in 0 .. points_register[0].len() {
			if is_low_point(x, y, &points_register) {
				basins.push((x, y));
			}
		}
	}

	// println!("1, 0 [{}]: {:?}", points_register[0][1], points_flowing_to_me(1, 0, &points_register));
	// println!("{}", point_flows_to_me((1, 0), (9, 0), &points_register));
	// println!("{}", points_register[0][1]);
	// println!("{:?}", basins[1]);
	// println!("{:?}", points_flowing_to_me(basins[1].0, basins[1].1, &points_register).len());
	// println!("{:?}", points_flowing_to_me(basins[1].0, basins[1].1, &points_register).iter().map(|l| (l.0, l.1, points_register[l.1][l.0])).collect::<Vec<(usize, usize, usize)>>());
	"hi".to_string()
}

#[derive(Debug)]
struct Basin {
	low_point: (usize, usize),
	points_in_basin: usize
}

fn is_low_point(_x: usize, _y: usize, points_register: &Vec<Vec<usize>>) -> bool {

	let mut neighbor_bools: Vec<bool> = Vec::new();
	let neighbors = get_neighbors(_x, _y, points_register);

	// println!("{}", neighbors.len());

	for coord in neighbors {
		let x = coord.0;
		let y = coord.1;

		let p_val = points_register[_y][_x];
		let n_val = points_register[y][x];

		if p_val >= n_val {
			neighbor_bools.push(true);
		} else {
			neighbor_bools.push(false);
		}
	}
	neighbor_bools.into_iter().filter(|l| *l ).collect::<Vec<bool>>().len() < 1
}


// confirmed working
fn get_neighbors(x: usize, y: usize, points_register: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {

	let mut neighbors: Vec<(usize, usize)> = Vec::new();

	if y > 0 {
		neighbors.push((x, y - 1));
	}

	if x > 0 {
		neighbors.push((x - 1, y));
	}

	if y < points_register.len() - 1 {
		neighbors.push((x, y + 1));
	}

	if x < points_register[y].len() - 1 {
		neighbors.push((x + 1, y));
	}

	neighbors
}


fn point_flows_to_me(point: (usize, usize), neighbor: (usize, usize), points_register: &Vec<Vec<usize>>) -> bool {
	let p_val = points_register[point.1][point.0];
	let n_val = points_register[neighbor.1][neighbor.0];

	let deep_neighbors = get_neighbors(neighbor.0, neighbor.1, points_register);

	let mut flows_to_me = p_val < n_val;
	
	for deep_neighbor in deep_neighbors {
		if points_register[deep_neighbor.1][deep_neighbor.0] <= n_val  && !(deep_neighbor.0 == point.0 && deep_neighbor.1 == point.1) {
			flows_to_me = false;
		}
	}

	flows_to_me
}

