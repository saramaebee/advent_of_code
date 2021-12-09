use std::{fmt::Debug, collections::HashSet};

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

	let mut basin_counts: Vec<usize> = Vec::new();
	for basin in basins {
		let flowing_to_me = points_flowing_to_me(basin.0, basin.1, &points_register);
		let mut x = flowing_to_me.iter().filter(|k| points_register[k.1][k.0] < 9).collect::<Vec<&(usize, usize)>>();
		let set: HashSet<_> = x.drain(..).collect(); // dedup
		x.extend(set.into_iter());
		basin_counts.push(x.len());
	}

	basin_counts.sort();

	basin_counts = basin_counts.into_iter().rev().collect();

	let largest_basins = basin_counts.drain(0..3).collect::<Vec<usize>>().into_iter().reduce(|a, b| a * b).unwrap();

	largest_basins.to_string()
}

#[derive(Debug)]
struct Basin {
	low_point: (usize, usize),
	points_in_basin: usize
}

fn is_low_point(_x: usize, _y: usize, points_register: &Vec<Vec<usize>>) -> bool {

	let mut neighbor_bools: Vec<bool> = Vec::new();
	let neighbors = get_neighbors(_x, _y, points_register);

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

fn points_flowing_to_me(x: usize, y: usize, points_register: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
	let mut flowing_to_me: Vec<(usize, usize)> = vec![(x, y)];
	let neighbors = get_neighbors(x, y, points_register);

	for i in 0 .. neighbors.len() {
		let neighbor_coords = neighbors[i];
		if point_flows_to_me((x, y), neighbor_coords, points_register) {
			flowing_to_me.append(&mut (points_flowing_to_me(neighbor_coords.0, neighbor_coords.1, points_register)));
		}
	}

	flowing_to_me
}

fn point_flows_to_me(point: (usize, usize), neighbor: (usize, usize), points_register: &Vec<Vec<usize>>) -> bool {
	let p_val = points_register[point.1][point.0];
	let n_val = points_register[neighbor.1][neighbor.0];

	let deep_neighbors = get_neighbors(neighbor.0, neighbor.1, points_register);

	let mut flows_to_me = p_val < n_val;
	
	for deep_neighbor in deep_neighbors {
		if points_register[deep_neighbor.1][deep_neighbor.0] < p_val  && !(deep_neighbor.0 == point.0 && deep_neighbor.1 == point.1) {
			flows_to_me = false;
		}
	}

	flows_to_me
}
