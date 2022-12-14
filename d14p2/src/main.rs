use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

impl Coordinate {
	fn move_toward(&self, other: &Self) -> Self {
		let x = match self.x.cmp(&other.x) {
			Ordering::Equal => self.x,
			Ordering::Less => self.x + 1,
			Ordering::Greater => self.x - 1,
		};
		let y = match self.y.cmp(&other.y) {
			Ordering::Equal => self.y,
			Ordering::Less => self.y + 1,
			Ordering::Greater => self.y - 1,
		};
		Self { x, y }
	}

	fn descend(&self) -> [Self; 3] {
		let down = Coordinate {
			x: self.x,
			y: self.y + 1,
		};
		let down_left = Coordinate {
			x: self.x - 1,
			y: self.y + 1,
		};
		let down_right = Coordinate {
			x: self.x + 1,
			y: self.y + 1,
		};
		[down, down_left, down_right]
	}
}

#[derive(Eq, PartialEq)]
enum ObjectType {
	Sand,
	Rock,
}

fn main() -> Result<(), Box<dyn Error>> {
	let paths: Vec<Vec<Coordinate>> = {
		let input_string = fs::read_to_string("input.txt")?;
		input_string
			.lines()
			.filter(|s| !s.is_empty())
			.map(|line| {
				line.split(" -> ")
					.map(|point| {
						let mut parts = point.split(',');
						let x = parts.next().unwrap();
						let y = parts.next().unwrap();
						assert!(parts.next().is_none());
						let x: u32 = x.parse().unwrap();
						let y: u32 = y.parse().unwrap();
						Coordinate { x, y }
					})
					.collect()
			})
			.collect()
	};

	let mut cave_objects: HashMap<Coordinate, ObjectType> = HashMap::new();

	for path in paths.iter() {
		let mut previous_point: Option<Coordinate> = None;
		for point in path.iter() {
			if let Some(prev_point) = previous_point.take() {
				let mut next_point = prev_point;
				loop {
					next_point = next_point.move_toward(point);
					cave_objects.insert(next_point, ObjectType::Rock);
					if next_point == *point {
						break;
					}
				}
			} else {
				cave_objects.insert(*point, ObjectType::Rock);
			}
			previous_point = Some(*point);
		}
	}

	let lowest_rock = paths.iter().flatten().map(|coord| coord.y).max().unwrap();

	let floor = lowest_rock + 2;

	let mut sand_position = Coordinate { x: 500, y: 0 };
	'sand_fall: loop {
		let next_sand_positions = sand_position.descend();
		for next_coord in next_sand_positions.iter() {
			if next_coord.y == floor {
				break;
			}
			if !cave_objects.contains_key(next_coord) {
				sand_position = *next_coord;
				continue 'sand_fall;
			}
		}
		cave_objects.insert(sand_position, ObjectType::Sand);
		if sand_position.x == 500 && sand_position.y == 0 {
			break;
		}
		sand_position = Coordinate { x: 500, y: 0 };
	}

	let sand_count = cave_objects.values().filter(|o| **o == ObjectType::Sand).count();
	println!("{}", sand_count);

	Ok(())
}
