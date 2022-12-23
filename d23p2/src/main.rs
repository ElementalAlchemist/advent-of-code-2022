use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	fn next_in_direction(&self, direction: &Direction) -> Self {
		match direction {
			Direction::North => Coordinate {
				x: self.x,
				y: self.y - 1,
			},
			Direction::South => Coordinate {
				x: self.x,
				y: self.y + 1,
			},
			Direction::West => Coordinate {
				x: self.x - 1,
				y: self.y,
			},
			Direction::East => Coordinate {
				x: self.x + 1,
				y: self.y,
			},
		}
	}

	fn fan_in_direction(&self, direction: &Direction) -> [Self; 3] {
		let (next_dir_left, next_dir_right) = match direction {
			Direction::North => (Direction::West, Direction::East),
			Direction::South => (Direction::West, Direction::East),
			Direction::West => (Direction::North, Direction::South),
			Direction::East => (Direction::North, Direction::South),
		};

		let next_coordinate = self.next_in_direction(direction);
		let next_coord_left = next_coordinate.next_in_direction(&next_dir_left);
		let next_coord_right = next_coordinate.next_in_direction(&next_dir_right);
		[next_coord_left, next_coordinate, next_coord_right]
	}

	fn all_surrounding_coords(&self) -> Vec<Self> {
		let mut coords = Vec::new();
		for x in -1..=1 {
			for y in -1..=1 {
				if x != 0 || y != 0 {
					coords.push(Coordinate {
						x: self.x + x,
						y: self.y + y,
					});
				}
			}
		}
		coords
	}
}

enum Direction {
	North,
	South,
	West,
	East,
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut elves: HashSet<Coordinate> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut coordinates: HashSet<Coordinate> = HashSet::new();

		for (line_index, line) in input_string.lines().enumerate() {
			let y: i32 = line_index.try_into().unwrap();
			for (char_index, c) in line.chars().enumerate() {
				if c == '#' {
					let x: i32 = char_index.try_into().unwrap();
					coordinates.insert(Coordinate { x, y });
				}
			}
		}

		coordinates
	};

	let mut directions: VecDeque<Direction> =
		vec![Direction::North, Direction::South, Direction::West, Direction::East]
			.drain(..)
			.collect();

	let mut round = 0;
	loop {
		round += 1;
		let mut move_made = false;
		let mut moves: HashMap<Coordinate, Coordinate> = HashMap::new();
		for elf_coord in elves.iter() {
			let surroundings = elf_coord.all_surrounding_coords();
			if surroundings.iter().filter(|coord| elves.contains(*coord)).count() > 0 {
				for direction in directions.iter() {
					let dir_coords = elf_coord.fan_in_direction(direction);
					if dir_coords.iter().filter(|coord| elves.contains(*coord)).count() == 0 {
						let move_dest = elf_coord.next_in_direction(direction);
						moves.insert(elf_coord.clone(), move_dest);
						break;
					}
				}
			}
		}

		let mut dest_counts: HashMap<Coordinate, u32> = HashMap::new();
		for move_dest in moves.values() {
			*dest_counts.entry(move_dest.clone()).or_default() += 1;
		}

		for (elf_move_start, elf_move_end) in moves.iter().filter(|(_, dest)| *dest_counts.get(*dest).unwrap() == 1) {
			elves.remove(elf_move_start);
			elves.insert(elf_move_end.clone());
			move_made = true;
		}

		let shift_direction = directions.pop_front().unwrap();
		directions.push_back(shift_direction);

		if !move_made {
			break;
		}
	}

	println!("{}", round);

	Ok(())
}
