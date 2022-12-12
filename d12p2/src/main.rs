use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn left(&self) -> Option<Self> {
		if self.x > 0 {
			Some(Self {
				x: self.x - 1,
				y: self.y,
			})
		} else {
			None
		}
	}

	fn up(&self) -> Option<Self> {
		if self.y > 0 {
			Some(Self {
				x: self.x,
				y: self.y - 1,
			})
		} else {
			None
		}
	}

	fn right(&self) -> Self {
		Self {
			x: self.x + 1,
			y: self.y,
		}
	}

	fn down(&self) -> Self {
		Self {
			x: self.x,
			y: self.y + 1,
		}
	}
}

#[derive(Clone, Eq, PartialEq)]
struct PathProgress {
	coord: Coordinate,
	steps_taken: u32,
}

impl Ord for PathProgress {
	fn cmp(&self, other: &Self) -> Ordering {
		self.steps_taken
			.cmp(&other.steps_taken)
			.then_with(|| self.coord.cmp(&other.coord))
	}
}

impl PartialOrd for PathProgress {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (end_coord, height_map) = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut end_coord: Option<Coordinate> = None;
		let height_map: Vec<Vec<usize>> = input_string
			.lines()
			.enumerate()
			.map(|(line_index, line)| {
				line.chars()
					.enumerate()
					.map(|(c_index, c)| match c {
						'S' => 0,
						'E' => {
							end_coord = Some(Coordinate {
								x: c_index,
								y: line_index,
							});
							25
						}
						_ => (c as usize) - 97,
					})
					.collect()
			})
			.collect();
		(end_coord.unwrap(), height_map)
	};

	let mut paths = BinaryHeap::new();
	for (y, row) in height_map.iter().enumerate() {
		for (x, height) in row.iter().enumerate() {
			if *height == 0 {
				paths.push(Reverse(PathProgress {
					coord: Coordinate { x, y },
					steps_taken: 0,
				}));
			}
		}
	}

	let mut visited_coords: HashSet<Coordinate> = HashSet::new();

	let mut steps_taken = 0;
	while let Some(Reverse(path)) = paths.pop() {
		if path.coord == end_coord {
			steps_taken = path.steps_taken;
			break;
		}
		if visited_coords.contains(&path.coord) {
			continue;
		}

		let steps_taken = path.steps_taken + 1;
		visited_coords.insert(path.coord);
		let current_height = height_map[path.coord.y][path.coord.x];

		if let Some(left_coord) = path.coord.left() {
			if height_map[left_coord.y][left_coord.x] <= current_height + 1 && !visited_coords.contains(&left_coord) {
				paths.push(Reverse(PathProgress {
					coord: left_coord,
					steps_taken,
				}));
			}
		}

		if let Some(up_coord) = path.coord.up() {
			if height_map[up_coord.y][up_coord.x] <= current_height + 1 && !visited_coords.contains(&up_coord) {
				paths.push(Reverse(PathProgress {
					coord: up_coord,
					steps_taken,
				}));
			}
		}

		let right_coord = path.coord.right();
		if right_coord.x < height_map[right_coord.y].len()
			&& height_map[right_coord.y][right_coord.x] <= current_height + 1
			&& !visited_coords.contains(&right_coord)
		{
			paths.push(Reverse(PathProgress {
				coord: right_coord,
				steps_taken,
			}));
		}

		let down_coord = path.coord.down();
		if down_coord.y < height_map.len()
			&& height_map[down_coord.y][down_coord.x] <= current_height + 1
			&& !visited_coords.contains(&down_coord)
		{
			paths.push(Reverse(PathProgress {
				coord: down_coord,
				steps_taken,
			}));
		}
	}

	println!("{}", steps_taken);

	Ok(())
}
