use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

#[derive(Eq, PartialEq)]
enum SpaceType {
	Open,
	Wall,
}

enum FacingDirection {
	Up,
	Right,
	Down,
	Left,
}

impl FacingDirection {
	fn value(&self) -> u32 {
		match self {
			Self::Right => 0,
			Self::Down => 1,
			Self::Left => 2,
			Self::Up => 3,
		}
	}
}

enum TurnDirection {
	Left,
	Right,
}

impl TurnDirection {
	fn turn_facing_direction(&self, facing: &FacingDirection) -> FacingDirection {
		match self {
			Self::Left => match facing {
				FacingDirection::Up => FacingDirection::Left,
				FacingDirection::Right => FacingDirection::Up,
				FacingDirection::Down => FacingDirection::Right,
				FacingDirection::Left => FacingDirection::Down,
			},
			Self::Right => match facing {
				FacingDirection::Up => FacingDirection::Right,
				FacingDirection::Right => FacingDirection::Down,
				FacingDirection::Down => FacingDirection::Left,
				FacingDirection::Left => FacingDirection::Up,
			},
		}
	}
}

enum Instruction {
	Travel(u32),
	Turn(TurnDirection),
}

fn main() -> Result<(), Box<dyn Error>> {
	let (map, instructions): (HashMap<Coordinate, SpaceType>, Vec<Instruction>) = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut map = HashMap::new();
		let mut instructions = Vec::new();

		let mut instructions_line = false;
		for (line_index, line) in input_string.lines().enumerate() {
			if instructions_line {
				let mut number = String::new();
				for c in line.chars() {
					if c.is_ascii_digit() {
						number.push(c);
					} else {
						let steps = number.parse()?;
						instructions.push(Instruction::Travel(steps));
						number.clear();
						let turn_direction = match c {
							'L' => TurnDirection::Left,
							'R' => TurnDirection::Right,
							_ => panic!("Unexpected turn direction {}", c),
						};
						instructions.push(Instruction::Turn(turn_direction));
					}
				}
				if !number.is_empty() {
					let steps = number.parse()?;
					instructions.push(Instruction::Travel(steps));
				}
				break;
			}
			if line.is_empty() {
				instructions_line = true;
				continue;
			}
			for (char_index, c) in line.chars().enumerate() {
				let char_index: u32 = char_index.try_into().unwrap();
				let line_index: u32 = line_index.try_into().unwrap();
				match c {
					'#' => {
						let coord = Coordinate {
							x: char_index + 1,
							y: line_index + 1,
						};
						map.insert(coord, SpaceType::Wall);
					}
					'.' => {
						let coord = Coordinate {
							x: char_index + 1,
							y: line_index + 1,
						};
						map.insert(coord, SpaceType::Open);
					}
					_ => continue,
				}
			}
		}

		(map, instructions)
	};

	let mut current_coord = Coordinate { x: 1, y: 1 };
	let mut current_direction = FacingDirection::Right;

	for instruction in instructions {
		match instruction {
			Instruction::Travel(steps) => {
				for _ in 0..steps {
					let mut next_coordinate = current_coord.clone();
					match current_direction {
						FacingDirection::Up => next_coordinate.y -= 1,
						FacingDirection::Right => next_coordinate.x += 1,
						FacingDirection::Down => next_coordinate.y += 1,
						FacingDirection::Left => next_coordinate.x -= 1,
					}
					current_coord = match map.get(&next_coordinate) {
						Some(space) => match space {
							SpaceType::Wall => break,
							SpaceType::Open => next_coordinate,
						},
						None => match current_direction {
							FacingDirection::Up => {
								let y = map
									.keys()
									.filter(|coord| coord.x == current_coord.x)
									.map(|coord| coord.y)
									.max()
									.unwrap();
								let next_coord = Coordinate { x: current_coord.x, y };
								if *map.get(&next_coord).unwrap() == SpaceType::Wall {
									break;
								} else {
									next_coord
								}
							}
							FacingDirection::Right => {
								let x = map
									.keys()
									.filter(|coord| coord.y == current_coord.y)
									.map(|coord| coord.x)
									.min()
									.unwrap();
								let next_coord = Coordinate { x, y: current_coord.y };
								if *map.get(&next_coord).unwrap() == SpaceType::Wall {
									break;
								} else {
									next_coord
								}
							}
							FacingDirection::Down => {
								let y = map
									.keys()
									.filter(|coord| coord.x == current_coord.x)
									.map(|coord| coord.y)
									.min()
									.unwrap();
								let next_coord = Coordinate { x: current_coord.x, y };
								if *map.get(&next_coord).unwrap() == SpaceType::Wall {
									break;
								} else {
									next_coord
								}
							}
							FacingDirection::Left => {
								let x = map
									.keys()
									.filter(|coord| coord.y == current_coord.y)
									.map(|coord| coord.x)
									.max()
									.unwrap();
								let next_coord = Coordinate { x, y: current_coord.y };
								if *map.get(&next_coord).unwrap() == SpaceType::Wall {
									break;
								} else {
									next_coord
								}
							}
						},
					};
					println!("({}, {})", current_coord.x, current_coord.y);
				}
			}
			Instruction::Turn(direction) => current_direction = direction.turn_facing_direction(&current_direction),
		}
	}

	let password_row = current_coord.y;
	let password_column = current_coord.x;
	let password_facing = current_direction.value();
	let password = password_row * 1000 + password_column * 4 + password_facing;
	println!("{} ({}, {}, {})", password, password_row, password_column, password_facing);

	Ok(())
}
