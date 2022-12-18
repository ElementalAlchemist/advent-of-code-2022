use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
	let cubes: HashSet<Coordinate> = {
		let input_string = fs::read_to_string("input.txt")?;
		input_string
			.lines()
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.collect()
	};

	let min_x = cubes.iter().map(|coord| coord.x).min().unwrap() - 1;
	let max_x = cubes.iter().map(|coord| coord.x).max().unwrap() + 1;
	let min_y = cubes.iter().map(|coord| coord.y).min().unwrap() - 1;
	let max_y = cubes.iter().map(|coord| coord.y).max().unwrap() + 1;
	let min_z = cubes.iter().map(|coord| coord.z).min().unwrap() - 1;
	let max_z = cubes.iter().map(|coord| coord.z).max().unwrap() + 1;

	let start_coordinate = Coordinate {
		x: min_x,
		y: min_y,
		z: min_z,
	};
	let mut visited: HashSet<Coordinate> = HashSet::new();

	let mut search_next = vec![start_coordinate];
	let mut exterior_sides = 0;

	while !search_next.is_empty() {
		let mut search_after = Vec::new();
		for search_coord in search_next.iter() {
			let adjacent = search_coord.adjacent();
			for adjacent_coord in adjacent.iter() {
				if cubes.contains(adjacent_coord) {
					exterior_sides += 1;
					continue;
				}
				if visited.contains(adjacent_coord)
					|| !(min_x..=max_x).contains(&adjacent_coord.x)
					|| !(min_y..=max_y).contains(&adjacent_coord.y)
					|| !(min_x..=max_z).contains(&adjacent_coord.z)
				{
					continue;
				}
				visited.insert(*adjacent_coord);
				search_after.push(*adjacent_coord);
			}
		}
		search_next = search_after;
	}

	println!("{}", exterior_sides);

	Ok(())
}
