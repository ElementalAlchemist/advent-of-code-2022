use std::collections::HashSet;
use std::error::Error;
use std::fs;

enum PushDirection {
	Left,
	Right,
}

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

impl Coordinate {
	fn move_left(&self) -> Self {
		Self {
			x: self.x - 1,
			y: self.y,
		}
	}

	fn move_right(&self) -> Self {
		Self {
			x: self.x + 1,
			y: self.y,
		}
	}

	fn move_down(&self) -> Self {
		Self {
			x: self.x,
			y: self.y - 1,
		}
	}
}

#[derive(Clone, Copy)]
enum RockType {
	StickHoriz,
	Plus,
	L,
	StickVert,
	Square,
}

impl RockType {
	fn get_coords(&self) -> Vec<Coordinate> {
		match self {
			Self::StickHoriz => vec![
				Coordinate { x: 0, y: 0 },
				Coordinate { x: 1, y: 0 },
				Coordinate { x: 2, y: 0 },
				Coordinate { x: 3, y: 0 },
			],
			Self::Plus => vec![
				Coordinate { x: 1, y: 2 },
				Coordinate { x: 0, y: 1 },
				Coordinate { x: 1, y: 1 },
				Coordinate { x: 2, y: 1 },
				Coordinate { x: 1, y: 0 },
			],
			Self::L => vec![
				Coordinate { x: 2, y: 2 },
				Coordinate { x: 2, y: 1 },
				Coordinate { x: 0, y: 0 },
				Coordinate { x: 1, y: 0 },
				Coordinate { x: 2, y: 0 },
			],
			Self::StickVert => vec![
				Coordinate { x: 0, y: 3 },
				Coordinate { x: 0, y: 2 },
				Coordinate { x: 0, y: 1 },
				Coordinate { x: 0, y: 0 },
			],
			Self::Square => vec![
				Coordinate { x: 0, y: 1 },
				Coordinate { x: 1, y: 1 },
				Coordinate { x: 0, y: 0 },
				Coordinate { x: 1, y: 0 },
			],
		}
	}

	fn get_types() -> Vec<Self> {
		vec![Self::StickHoriz, Self::Plus, Self::L, Self::StickVert, Self::Square]
	}
}

struct Rock {
	rock: RockType,
	x_offset: u32,
	y_offset: u32,
}

impl Rock {
	fn new(rock: RockType, y_offset: u32) -> Self {
		let x_offset = 2;
		Self {
			rock,
			x_offset,
			y_offset,
		}
	}

	fn get_coords(&self) -> Vec<Coordinate> {
		let mut coords = self.rock.get_coords();
		for coord in coords.iter_mut() {
			coord.x += self.x_offset;
			coord.y += self.y_offset;
		}
		coords
	}

	fn width(&self) -> u32 {
		let coords = self.rock.get_coords();
		coords.iter().map(|coord| coord.x).max().unwrap() + 1
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let pushes: Vec<PushDirection> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut pushes = Vec::new();
		for c in input_string.chars() {
			match c {
				'<' => pushes.push(PushDirection::Left),
				'>' => pushes.push(PushDirection::Right),
				_ => (),
			}
		}
		pushes
	};

	let rock_types = RockType::get_types();

	let mut push_direction_iter = pushes.iter().cycle();
	let mut rock_type_iter = rock_types.iter().cycle();

	let mut max_rock_height = 0;
	let mut fitted_rock_map: HashSet<Coordinate> = HashSet::new();

	for _ in 0..2022 {
		let mut rock = Rock::new(*rock_type_iter.next().unwrap(), max_rock_height + 3);
		let rock_width = rock.width();

		loop {
			let direction = push_direction_iter.next().unwrap();
			match direction {
				PushDirection::Left => {
					if rock.x_offset > 0 {
						let rock_coords = rock.get_coords();
						let mut can_move = true;
						for coord in rock_coords.iter() {
							let moved_coord = coord.move_left();
							if fitted_rock_map.contains(&moved_coord) {
								can_move = false;
								break;
							}
						}
						if can_move {
							rock.x_offset -= 1;
						}
					}
				}
				PushDirection::Right => {
					if rock.x_offset + rock_width < 7 {
						let rock_coords = rock.get_coords();
						let mut can_move = true;
						for coord in rock_coords.iter() {
							let moved_coord = coord.move_right();
							if fitted_rock_map.contains(&moved_coord) {
								can_move = false;
								break;
							}
						}
						if can_move {
							rock.x_offset += 1;
						}
					}
				}
			}

			let coords = rock.get_coords();
			let mut hit_bottom = false;
			for coord in coords.iter() {
				if coord.y == 0 {
					hit_bottom = true;
					break;
				}
				let below_coord = coord.move_down();
				if fitted_rock_map.contains(&below_coord) {
					hit_bottom = true;
					break;
				}
			}

			if hit_bottom {
				let mut coords = rock.get_coords();
				for coord in coords.drain(..) {
					max_rock_height = max_rock_height.max(coord.y + 1);
					fitted_rock_map.insert(coord);
				}
				break;
			}
			rock.y_offset -= 1;
		}
	}

	println!("{}", max_rock_height);

	Ok(())
}
