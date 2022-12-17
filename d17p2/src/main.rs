use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::Cycle;
use std::slice::Iter;

const MAX_ITERATIONS: usize = 1_000_000_000_000;

enum PushDirection {
	Left,
	Right,
}

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
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
	x_offset: usize,
	y_offset: usize,
}

impl Rock {
	fn new(rock: RockType, y_offset: usize) -> Self {
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

	fn width(&self) -> usize {
		let coords = self.rock.get_coords();
		coords.iter().map(|coord| coord.x).max().unwrap() + 1
	}
}

struct CyclePosition {
	cycle_end: usize,
	current_position: usize,
}

fn rock_fall_iteration(
	rock_type_iter: &mut Cycle<Iter<RockType>>,
	max_rock_height: &mut usize,
	push_direction_iter: &mut Cycle<Iter<PushDirection>>,
	fitted_rock_map: &mut HashSet<Coordinate>,
) {
	let mut rock = Rock::new(*rock_type_iter.next().unwrap(), *max_rock_height + 3);
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
				*max_rock_height = (*max_rock_height).max(coord.y + 1);
				fitted_rock_map.insert(coord);
			}
			break;
		}
		rock.y_offset -= 1;
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

	let big_iteration_size = pushes.len() * rock_types.len();

	// Let the initial build-up happen before the cycle starts
	// My input has one big iteration before the cycle begins; the example has 2
	// I just used 2 universally here because all we need is to get into when the cycles start
	for _ in 0..2 {
		for _ in 0..big_iteration_size {
			rock_fall_iteration(
				&mut rock_type_iter,
				&mut max_rock_height,
				&mut push_direction_iter,
				&mut fitted_rock_map,
			);
		}
	}

	let mut past_iteration_additions: Vec<usize> = Vec::new();
	let mut cycle_positions: Vec<CyclePosition> = Vec::new();
	let mut previous_height = max_rock_height;

	for _ in 0..big_iteration_size {
		rock_fall_iteration(
			&mut rock_type_iter,
			&mut max_rock_height,
			&mut push_direction_iter,
			&mut fitted_rock_map,
		);
	}
	past_iteration_additions.push(max_rock_height - previous_height);
	previous_height = max_rock_height;

	let cycle_size = 'cycle: loop {
		for _ in 0..big_iteration_size {
			rock_fall_iteration(
				&mut rock_type_iter,
				&mut max_rock_height,
				&mut push_direction_iter,
				&mut fitted_rock_map,
			);
		}
		let added_this_cycle = max_rock_height - previous_height;
		previous_height = max_rock_height;

		let mut new_cycle_positions = Vec::new();
		for current_cycle in cycle_positions.iter() {
			let next_cycle = current_cycle.current_position + 1;
			let cycle_complete = next_cycle == current_cycle.cycle_end;
			let next_compare = if cycle_complete {
				past_iteration_additions[0]
			} else {
				past_iteration_additions[next_cycle]
			};
			if added_this_cycle == next_compare {
				if cycle_complete {
					break 'cycle current_cycle.cycle_end;
				}
				let next_cycle = CyclePosition {
					current_position: next_cycle,
					cycle_end: current_cycle.cycle_end,
				};
				new_cycle_positions.push(next_cycle);
			}
		}
		if added_this_cycle == past_iteration_additions[0] {
			let new_cycle = CyclePosition {
				current_position: 0,
				cycle_end: past_iteration_additions.len(),
			};
			new_cycle_positions.push(new_cycle);
		}
		cycle_positions = new_cycle_positions;
		past_iteration_additions.push(added_this_cycle);
	};

	let cycle_rock_height: usize = past_iteration_additions.iter().take(cycle_size).sum();

	let num_big_iterations = MAX_ITERATIONS / big_iteration_size;
	let remaining_iterations = MAX_ITERATIONS % big_iteration_size;

	let num_cycles = num_big_iterations / cycle_size;
	let remaining_big_iterations = num_big_iterations % cycle_size;

	let mut rock_type_iter = rock_types.iter().cycle();
	let mut push_direction_iter = pushes.iter().cycle();
	fitted_rock_map.clear();
	max_rock_height = 0;

	for _ in 0..(remaining_big_iterations * big_iteration_size + remaining_iterations) {
		rock_fall_iteration(
			&mut rock_type_iter,
			&mut max_rock_height,
			&mut push_direction_iter,
			&mut fitted_rock_map,
		);
	}
	max_rock_height += cycle_rock_height * num_cycles;

	println!("{}", max_rock_height);

	Ok(())
}
