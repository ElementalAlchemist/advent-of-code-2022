use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Eq, Hash, PartialEq)]
enum TraversePosition {
	Start,
	Coord(Coordinate),
}

fn main() -> Result<(), Box<dyn Error>> {
	let (map_width, map_height, mut blizzards) = {
		let input_string = fs::read_to_string("input.txt")?;
		let line_count = input_string.lines().count();
		let mut puzzle_height: usize = 0;
		let mut puzzle_width: usize = 0;
		let mut blizzards: HashMap<Coordinate, Direction> = HashMap::new();
		for (line_num, line) in input_string.lines().take(line_count - 1).skip(1).enumerate() {
			let line = line.strip_prefix('#').unwrap();
			let line = line.strip_suffix('#').unwrap();

			puzzle_height = line_num + 1;
			puzzle_width = line.len();

			for (char_index, c) in line.chars().enumerate() {
				let direction = match c {
					'^' => Direction::Up,
					'<' => Direction::Left,
					'v' => Direction::Down,
					'>' => Direction::Right,
					_ => continue,
				};
				let coordinate = Coordinate {
					x: char_index,
					y: line_num,
				};
				blizzards.insert(coordinate, direction);
			}
		}

		(puzzle_width, puzzle_height, blizzards)
	};

	let mut up_blizzards: Vec<HashSet<usize>> = vec![HashSet::new(); map_width];
	let mut down_blizzards: Vec<HashSet<usize>> = vec![HashSet::new(); map_width];
	let mut left_blizzards: Vec<HashSet<usize>> = vec![HashSet::new(); map_height];
	let mut right_blizzards: Vec<HashSet<usize>> = vec![HashSet::new(); map_height];

	for (coord, direction) in blizzards.drain() {
		match direction {
			Direction::Up => up_blizzards[coord.x].insert(coord.y),
			Direction::Down => down_blizzards[coord.x].insert(coord.y),
			Direction::Left => left_blizzards[coord.y].insert(coord.x),
			Direction::Right => right_blizzards[coord.y].insert(coord.x),
		};
	}

	// Remove mutability
	let up_blizzards = up_blizzards;
	let down_blizzards = down_blizzards;
	let left_blizzards = left_blizzards;
	let right_blizzards = right_blizzards;

	let has_up_blizzard = |coord: &Coordinate, minutes: usize| {
		let new_y = (coord.y + minutes) % map_height;
		up_blizzards[coord.x].contains(&new_y)
	};

	let has_down_blizzard = |coord: &Coordinate, minutes: usize| {
		let minutes = minutes % map_height;
		let new_y = (coord.y + map_height - minutes) % map_height;
		down_blizzards[coord.x].contains(&new_y)
	};

	let has_left_blizzard = |coord: &Coordinate, minutes: usize| {
		let new_x = (coord.x + minutes) % map_width;
		left_blizzards[coord.y].contains(&new_x)
	};

	let has_right_blizzard = |coord: &Coordinate, minutes: usize| {
		let minutes = minutes % map_width;
		let new_x = (coord.x + map_width - minutes) % map_width;
		right_blizzards[coord.y].contains(&new_x)
	};

	let has_blizzard = |coord: &Coordinate, minutes: usize| {
		has_up_blizzard(coord, minutes)
			|| has_down_blizzard(coord, minutes)
			|| has_left_blizzard(coord, minutes)
			|| has_right_blizzard(coord, minutes)
	};

	let mut positions: HashSet<TraversePosition> = HashSet::new();
	positions.insert(TraversePosition::Start);

	let dest_coord = Coordinate {
		x: map_width - 1,
		y: map_height - 1,
	};
	let mut minutes = 0;
	'minutes: loop {
		minutes += 1;
		let mut new_positions = HashSet::new();
		for position in positions.iter() {
			match position {
				TraversePosition::Start => {
					let new_coord = Coordinate { x: 0, y: 0 };
					if has_blizzard(&new_coord, minutes) {
						new_positions.insert(TraversePosition::Start);
					} else {
						new_positions.insert(TraversePosition::Coord(new_coord));
					}
				}
				TraversePosition::Coord(start_coord) => {
					if *start_coord == dest_coord {
						break 'minutes;
					}

					if start_coord.y > 0 {
						let up_coord = Coordinate {
							x: start_coord.x,
							y: start_coord.y - 1,
						};
						if !has_blizzard(&up_coord, minutes) {
							new_positions.insert(TraversePosition::Coord(up_coord));
						}
					}

					if start_coord.x > 0 {
						let left_coord = Coordinate {
							x: start_coord.x - 1,
							y: start_coord.y,
						};
						if !has_blizzard(&left_coord, minutes) {
							new_positions.insert(TraversePosition::Coord(left_coord));
						}
					}

					let down_coord = Coordinate {
						x: start_coord.x,
						y: (start_coord.y + 1) % map_height,
					};
					if !has_blizzard(&down_coord, minutes) {
						new_positions.insert(TraversePosition::Coord(down_coord));
					}

					let right_coord = Coordinate {
						x: (start_coord.x + 1) % map_width,
						y: start_coord.y,
					};
					if !has_blizzard(&right_coord, minutes) {
						new_positions.insert(TraversePosition::Coord(right_coord));
					}

					if !has_blizzard(start_coord, minutes) {
						new_positions.insert(TraversePosition::Coord(start_coord.clone()));
					}
				}
			}
		}
		positions = new_positions;
	}

	println!("{}", minutes);

	Ok(())
}
