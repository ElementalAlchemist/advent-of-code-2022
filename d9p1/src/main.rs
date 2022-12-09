use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::str::FromStr;

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl FromStr for Direction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"U" => Self::Up,
			"R" => Self::Right,
			"D" => Self::Down,
			"L" => Self::Left,
			_ => panic!("Unsupported direction: {}", s),
		})
	}
}

struct Instruction {
	direction: Direction,
	distance: i32,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(' ');
		let direction = parts.next().unwrap();
		let direction: Direction = direction.parse().unwrap();

		let distance = parts.next().unwrap();
		let distance: i32 = distance.parse().unwrap();

		Ok(Self { direction, distance })
	}
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	fn move_in_direction(&mut self, direction: &Direction) {
		match direction {
			Direction::Up => self.y -= 1,
			Direction::Right => self.x += 1,
			Direction::Down => self.y += 1,
			Direction::Left => self.x -= 1,
		}
	}

	fn move_toward(&mut self, other_coordinate: &Self) {
		if self.x == other_coordinate.x {
			if self.y < other_coordinate.y - 1 {
				self.y += 1;
			} else if self.y > other_coordinate.y + 1 {
				self.y -= 1;
			}
		} else if self.y == other_coordinate.y {
			if self.x < other_coordinate.x - 1 {
				self.x += 1;
			} else if self.x > other_coordinate.x + 1 {
				self.x -= 1;
			}
		} else if self.x.abs_diff(other_coordinate.x) > 1 || self.y.abs_diff(other_coordinate.y) > 1 {
			if self.x < other_coordinate.x {
				self.x += 1;
			} else {
				self.x -= 1;
			}
			if self.y < other_coordinate.y {
				self.y += 1;
			} else {
				self.y -= 1;
			}
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let instructions = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut instructions: Vec<Instruction> = Vec::new();
		for instruction in input_string.lines().filter(|s| !s.is_empty()) {
			instructions.push(instruction.parse().unwrap());
		}
		instructions
	};

	let mut head_coordinate = Coordinate::default();
	let mut tail_coordinate = Coordinate::default();
	let mut tail_visited: HashSet<Coordinate> = HashSet::new();

	for instruction in instructions {
		for _ in 0..instruction.distance {
			head_coordinate.move_in_direction(&instruction.direction);
			tail_coordinate.move_toward(&head_coordinate);
			tail_visited.insert(tail_coordinate.clone());
		}
	}

	println!("{}", tail_visited.len());

	Ok(())
}
