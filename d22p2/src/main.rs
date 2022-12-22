use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

#[derive(Clone, Eq, PartialEq)]
enum SpaceType {
	Open,
	Wall,
	Teleport(TeleportDestination),
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
enum CubeFace {
	Top,
	Left,
	Front,
	Right,
	Bottom,
	Back,
}

enum Instruction {
	Travel(u32),
	Turn(TurnDirection),
}

#[derive(Clone, Eq, PartialEq)]
struct TeleportDestination {
	face: CubeFace,
	coord: Coordinate,
	facing: FacingDirection,
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

	// For now, this only handles the shape of the example and my input.
	// This also does some hardcoding because it's late and orienting cube faces seems hard right now.
	let first_row_coords: u32 = map.keys().filter(|coord| coord.y == 1).count().try_into().unwrap();

	let is_example = first_row_coords == 4;
	if !is_example {
		assert_eq!(first_row_coords, 100);
	}

	let mut top_map = HashMap::new();
	let mut front_map = HashMap::new();
	let mut bottom_map = HashMap::new();
	let mut back_map = HashMap::new();
	let mut left_map = HashMap::new();
	let mut right_map = HashMap::new();

	let (side_len, top_tl, front_tl, bottom_tl, back_tl, left_tl, right_tl) = if is_example {
		(
			4,
			Coordinate { x: 9, y: 1 },
			Coordinate { x: 9, y: 5 },
			Coordinate { x: 9, y: 9 },
			Coordinate { x: 1, y: 5 },
			Coordinate { x: 5, y: 5 },
			Coordinate { x: 13, y: 9 },
		)
	} else {
		(
			50,
			Coordinate { x: 51, y: 1 },
			Coordinate { x: 51, y: 51 },
			Coordinate { x: 51, y: 101 },
			Coordinate { x: 1, y: 151 },
			Coordinate { x: 1, y: 101 },
			Coordinate { x: 101, y: 1 },
		)
	};

	for x in top_tl.x..(top_tl.x + side_len) {
		for y in top_tl.y..(top_tl.y + side_len) {
			let coord = Coordinate { x, y };
			let space_type = map.get(&coord).unwrap().clone();
			top_map.insert(coord, space_type);
		}
	}
	for y in top_tl.y..(top_tl.y + side_len) {
		let coord = Coordinate { x: top_tl.x - 1, y };
		let face = CubeFace::Left;
		let (facing, destination_coord) = if is_example {
			(
				FacingDirection::Down,
				Coordinate {
					x: left_tl.x + (y - top_tl.y),
					y: left_tl.y,
				},
			)
		} else {
			(
				FacingDirection::Right,
				Coordinate {
					x: left_tl.x,
					y: left_tl.y + side_len - (y + 1 - top_tl.y),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		top_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x: top_tl.x + side_len,
			y,
		};
		let face = CubeFace::Right;
		let (facing, destination_coord) = if is_example {
			(
				FacingDirection::Left,
				Coordinate {
					x: right_tl.x + side_len - 1,
					y: right_tl.y + side_len - (y + 1 - top_tl.y),
				},
			)
		} else {
			(
				FacingDirection::Right,
				Coordinate {
					x: right_tl.x,
					y
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		top_map.insert(coord, SpaceType::Teleport(destination));
	}
	for x in top_tl.x..(top_tl.x + side_len) {
		let coord = Coordinate { x, y: top_tl.y - 1 };
		let face = CubeFace::Back;
		let (facing, destination_coord) = if is_example {
			(
				FacingDirection::Down,
				Coordinate {
					x: back_tl.x + side_len - (x + 1 - top_tl.x),
					y: back_tl.y,
				},
			)
		} else {
			(
				FacingDirection::Right,
				Coordinate {
					x: back_tl.x,
					y: back_tl.y + (x - top_tl.x),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		top_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x,
			y: top_tl.y + side_len,
		};
		let face = CubeFace::Front;
		let (facing, destination_coord) = (FacingDirection::Down, Coordinate { x, y: front_tl.y });
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		top_map.insert(coord, SpaceType::Teleport(destination));
	}

	for x in front_tl.x..(front_tl.x + side_len) {
		for y in front_tl.y..(front_tl.y + side_len) {
			let coord = Coordinate { x, y };
			let space_type = map.get(&coord).unwrap().clone();
			front_map.insert(coord, space_type);
		}
	}
	for y in front_tl.y..(front_tl.y + side_len) {
		let coord = Coordinate { x: front_tl.x - 1, y };
		let face = CubeFace::Left;
		let (facing, destination_coord) = if is_example {
			(
				FacingDirection::Right,
				Coordinate {
					x: left_tl.x + side_len - 1,
					y,
				},
			)
		} else {
			(
				FacingDirection::Down,
				Coordinate {
					x: left_tl.x + (y - front_tl.y),
					y: left_tl.y,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		front_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x: front_tl.x + side_len,
			y,
		};
		let face = CubeFace::Right;
		let (facing, destination_coord) = if is_example {
			(
				FacingDirection::Down,
				Coordinate {
					x: right_tl.x + side_len - (y + 1 - front_tl.y),
					y: right_tl.y,
				},
			)
		} else {
			(
				FacingDirection::Up,
				Coordinate {
					x: right_tl.x + (y - front_tl.y),
					y: right_tl.y + side_len - 1,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		front_map.insert(coord, SpaceType::Teleport(destination));
	}
	for x in front_tl.x..(front_tl.x + side_len) {
		let coord = Coordinate { x, y: front_tl.y - 1 };
		let face = CubeFace::Top;
		let (facing, destination_coord) = (
			FacingDirection::Up,
			Coordinate {
				x,
				y: top_tl.y + side_len - 1,
			},
		);
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		front_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x,
			y: front_tl.y + side_len,
		};
		let face = CubeFace::Bottom;
		let (facing, destination_coord) = (FacingDirection::Down, Coordinate { x, y: bottom_tl.y });
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		front_map.insert(coord, SpaceType::Teleport(destination));
	}

	for x in bottom_tl.x..(bottom_tl.x + side_len) {
		for y in bottom_tl.y..(bottom_tl.y + side_len) {
			let coord = Coordinate { x, y };
			let space_type = map.get(&coord).unwrap().clone();
			bottom_map.insert(coord, space_type);
		}
	}
	for y in bottom_tl.y..(bottom_tl.y + side_len) {
		let coord = Coordinate { x: bottom_tl.x - 1, y };
		let face = CubeFace::Left;
		let (facing, destination_coord) = if is_example {
			(
				FacingDirection::Up,
				Coordinate {
					x: left_tl.x + (y - bottom_tl.y),
					y: left_tl.y + side_len - 1,
				},
			)
		} else {
			(
				FacingDirection::Left,
				Coordinate {
					x: left_tl.x + side_len - 1,
					y,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		bottom_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x: bottom_tl.x + side_len,
			y,
		};
		let face = CubeFace::Right;
		let (facing, destination_coord) = if is_example {
			(FacingDirection::Right, Coordinate { x: right_tl.x, y })
		} else {
			(
				FacingDirection::Left,
				Coordinate {
					x: right_tl.x + side_len - 1,
					y: right_tl.y + side_len - (y + 1 - bottom_tl.y),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		bottom_map.insert(coord, SpaceType::Teleport(destination));
	}
	for x in bottom_tl.x..(bottom_tl.x + side_len) {
		let coord = Coordinate { x, y: bottom_tl.y - 1 };
		let face = CubeFace::Front;
		let (facing, destination_coord) = (
			FacingDirection::Up,
			Coordinate {
				x,
				y: front_tl.y + side_len - 1,
			},
		);
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		bottom_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x,
			y: bottom_tl.y + side_len,
		};
		let face = CubeFace::Back;
		let (facing, destination_coord) = if is_example {
			(
				FacingDirection::Up,
				Coordinate {
					x: back_tl.x + side_len - (x + 1 - bottom_tl.x),
					y: back_tl.y + side_len - 1,
				},
			)
		} else {
			(
				FacingDirection::Left,
				Coordinate {
					x: back_tl.x + side_len - 1,
					y: back_tl.y + (x - bottom_tl.x),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		bottom_map.insert(coord, SpaceType::Teleport(destination));
	}

	for x in back_tl.x..(back_tl.x + side_len) {
		for y in back_tl.y..(back_tl.y + side_len) {
			let coord = Coordinate { x, y };
			let space_type = map.get(&coord).unwrap().clone();
			back_map.insert(coord, space_type);
		}
	}
	for y in back_tl.y..(back_tl.y + side_len) {
		let coord = Coordinate { x: back_tl.x - 1, y };
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Right,
				FacingDirection::Up,
				Coordinate {
					x: right_tl.x + (y - back_tl.y),
					y: right_tl.y + side_len - 1,
				},
			)
		} else {
			(
				CubeFace::Top,
				FacingDirection::Down,
				Coordinate {
					x: top_tl.x + (y - back_tl.y),
					y: top_tl.y,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		back_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x: back_tl.x + side_len,
			y,
		};
		let (face, facing, destination_coord) = if is_example {
			(CubeFace::Left, FacingDirection::Right, Coordinate { x: left_tl.x, y })
		} else {
			(
				CubeFace::Bottom,
				FacingDirection::Up,
				Coordinate {
					x: bottom_tl.x + (y - back_tl.y),
					y: bottom_tl.y + side_len - 1,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		back_map.insert(coord, SpaceType::Teleport(destination));
	}
	for x in back_tl.x..(back_tl.x + side_len) {
		let coord = Coordinate { x, y: back_tl.y - 1 };
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Top,
				FacingDirection::Down,
				Coordinate {
					x: top_tl.x + side_len - (x + 1 - back_tl.x),
					y: top_tl.y,
				},
			)
		} else {
			(
				CubeFace::Left,
				FacingDirection::Up,
				Coordinate {
					x,
					y: left_tl.y + side_len - 1,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		back_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x,
			y: back_tl.y + side_len,
		};
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Bottom,
				FacingDirection::Up,
				Coordinate {
					x: bottom_tl.x + side_len - (x + 1 - back_tl.x),
					y: bottom_tl.y + side_len - 1,
				},
			)
		} else {
			(
				CubeFace::Right,
				FacingDirection::Down,
				Coordinate {
					x: right_tl.x + (x - back_tl.x),
					y: right_tl.y,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		back_map.insert(coord, SpaceType::Teleport(destination));
	}

	for x in left_tl.x..(left_tl.x + side_len) {
		for y in left_tl.y..(left_tl.y + side_len) {
			let coord = Coordinate { x, y };
			let space_type = map.get(&coord).unwrap().clone();
			left_map.insert(coord, space_type);
		}
	}
	for y in left_tl.y..(left_tl.y + side_len) {
		let coord = Coordinate { x: left_tl.x - 1, y };
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Back,
				FacingDirection::Left,
				Coordinate {
					x: back_tl.x + side_len - 1,
					y,
				},
			)
		} else {
			(
				CubeFace::Top,
				FacingDirection::Right,
				Coordinate {
					x: top_tl.x,
					y: top_tl.y + side_len - (y + 1 - left_tl.y),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		left_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x: left_tl.x + side_len,
			y,
		};
		let (face, facing, destination_coord) = if is_example {
			(CubeFace::Front, FacingDirection::Right, Coordinate { x: front_tl.x, y })
		} else {
			(
				CubeFace::Bottom,
				FacingDirection::Right,
				Coordinate { x: bottom_tl.x, y },
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		left_map.insert(coord, SpaceType::Teleport(destination));
	}
	for x in left_tl.x..(left_tl.x + side_len) {
		let coord = Coordinate { x, y: left_tl.y - 1 };
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Top,
				FacingDirection::Right,
				Coordinate {
					x: top_tl.x,
					y: top_tl.y + (x - left_tl.x),
				},
			)
		} else {
			(
				CubeFace::Front,
				FacingDirection::Right,
				Coordinate {
					x: front_tl.x,
					y: front_tl.y + (x - left_tl.x),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		left_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x,
			y: left_tl.y + side_len,
		};
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Bottom,
				FacingDirection::Right,
				Coordinate {
					x: bottom_tl.x,
					y: bottom_tl.y + side_len - (x + 1 - left_tl.x),
				},
			)
		} else {
			(
				CubeFace::Back,
				FacingDirection::Down,
				Coordinate {
					x,
					y: back_tl.y,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		left_map.insert(coord, SpaceType::Teleport(destination));
	}

	for x in right_tl.x..(right_tl.x + side_len) {
		for y in right_tl.y..(right_tl.y + side_len) {
			let coord = Coordinate { x, y };
			let space_type = map.get(&coord).unwrap().clone();
			right_map.insert(coord, space_type);
		}
	}
	for y in right_tl.y..(right_tl.y + side_len) {
		let coord = Coordinate { x: right_tl.x - 1, y };
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Bottom,
				FacingDirection::Left,
				Coordinate {
					x: bottom_tl.x + side_len - 1,
					y,
				},
			)
		} else {
			(
				CubeFace::Top,
				FacingDirection::Left,
				Coordinate {
					x: top_tl.x + side_len - 1,
					y,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		right_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x: right_tl.x + side_len,
			y,
		};
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Top,
				FacingDirection::Left,
				Coordinate {
					x: top_tl.x + side_len - 1,
					y: top_tl.y + side_len - (y + 1 - right_tl.y),
				},
			)
		} else {
			(
				CubeFace::Bottom,
				FacingDirection::Left,
				Coordinate {
					x: bottom_tl.x + side_len - 1,
					y: bottom_tl.y + side_len - (y + 1 - right_tl.y),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		right_map.insert(coord, SpaceType::Teleport(destination));
	}
	for x in right_tl.x..(right_tl.x + side_len) {
		let coord = Coordinate { x, y: right_tl.y - 1 };
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Front,
				FacingDirection::Left,
				Coordinate {
					x: front_tl.x + side_len - 1,
					y: front_tl.y + side_len - (x + 1 - right_tl.x),
				},
			)
		} else {
			(
				CubeFace::Back,
				FacingDirection::Up,
				Coordinate {
					x: back_tl.x + (x - right_tl.x),
					y: back_tl.y + side_len - 1,
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		right_map.insert(coord, SpaceType::Teleport(destination));

		let coord = Coordinate {
			x,
			y: right_tl.y + side_len,
		};
		let (face, facing, destination_coord) = if is_example {
			(
				CubeFace::Back,
				FacingDirection::Right,
				Coordinate {
					x: back_tl.x,
					y: back_tl.y + side_len - (x + 1 - right_tl.x),
				},
			)
		} else {
			(
				CubeFace::Front,
				FacingDirection::Left,
				Coordinate {
					x: front_tl.x + side_len - 1,
					y: front_tl.y + (x - right_tl.x),
				},
			)
		};
		let destination = TeleportDestination {
			face,
			coord: destination_coord,
			facing,
		};
		right_map.insert(coord, SpaceType::Teleport(destination));
	}

	let mut current_coord = if is_example {
		Coordinate { x: 9, y: 1 }
	} else {
		Coordinate { x: 51, y: 1 }
	};
	let mut current_direction = FacingDirection::Right;
	let mut current_face = CubeFace::Top;

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
					let map = match current_face {
						CubeFace::Top => &top_map,
						CubeFace::Front => &front_map,
						CubeFace::Bottom => &bottom_map,
						CubeFace::Back => &back_map,
						CubeFace::Left => &left_map,
						CubeFace::Right => &right_map,
					};
					let (dest_face, dest_coord, dest_facing) = match map.get(&next_coordinate) {
						Some(space) => match space {
							SpaceType::Wall => break,
							SpaceType::Open => (current_face, next_coordinate, current_direction),
							SpaceType::Teleport(destination) => {
								let teleport_map = match destination.face {
									CubeFace::Top => &top_map,
									CubeFace::Front => &front_map,
									CubeFace::Bottom => &bottom_map,
									CubeFace::Back => &back_map,
									CubeFace::Left => &left_map,
									CubeFace::Right => &right_map,
								};
								println!("Teleport from {:?} to {:?} ({}, {})", current_face, destination.face, destination.coord.x, destination.coord.y);
								let space_type = teleport_map.get(&destination.coord).unwrap();
								match space_type {
									SpaceType::Open => (
										destination.face.clone(),
										destination.coord.clone(),
										destination.facing.clone(),
									),
									SpaceType::Wall => (current_face, current_coord, current_direction),
									SpaceType::Teleport(_) => unreachable!(),
								}
							}
						},
						None => unreachable!(),
					};
					current_face = dest_face;
					current_coord = dest_coord;
					current_direction = dest_facing;
				}
			}
			Instruction::Turn(direction) => current_direction = direction.turn_facing_direction(&current_direction),
		}
		println!("({}, {}) {:?}", current_coord.x, current_coord.y, current_direction);
	}

	let password_row = current_coord.y;
	let password_column = current_coord.x;
	let password_facing = current_direction.value();
	let password = password_row * 1000 + password_column * 4 + password_facing;
	println!(
		"{} ({}, {}, {})",
		password, password_row, password_column, password_facing
	);

	Ok(())
}
