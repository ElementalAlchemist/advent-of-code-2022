use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
	z: i32,
}

impl Coordinate {
	fn adjacent(&self) -> [Self; 6] {
		let up = Self {
			x: self.x,
			y: self.y + 1,
			z: self.z,
		};
		let down = Self {
			x: self.x,
			y: self.y - 1,
			z: self.z,
		};
		let left = Self {
			x: self.x - 1,
			y: self.y,
			z: self.z,
		};
		let right = Self {
			x: self.x + 1,
			y: self.y,
			z: self.z,
		};
		let front = Self {
			x: self.x,
			y: self.y,
			z: self.z - 1,
		};
		let back = Self {
			x: self.x,
			y: self.y,
			z: self.z + 1,
		};

		[up, down, left, right, front, back]
	}
}

impl FromStr for Coordinate {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(',');
		let x = parts.next().unwrap();
		let y = parts.next().unwrap();
		let z = parts.next().unwrap();
		assert!(parts.next().is_none());

		let x: i32 = x.parse()?;
		let y: i32 = y.parse()?;
		let z: i32 = z.parse()?;

		Ok(Self { x, y, z })
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let coordinates: HashSet<Coordinate> = {
		let input_string = fs::read_to_string("input.txt")?;
		input_string
			.lines()
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.collect()
	};

	let mut exposed_sides = 0;
	for coord in coordinates.iter() {
		let adjacent_coords = coord.adjacent();
		for adjacent_coord in adjacent_coords.iter() {
			if !coordinates.contains(adjacent_coord) {
				exposed_sides += 1;
			}
		}
	}

	println!("{}", exposed_sides);

	Ok(())
}
