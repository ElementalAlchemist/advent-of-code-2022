use std::collections::HashSet;
use std::error::Error;
use std::fs;

const MAX_TIME: u32 = 24;

struct Blueprint {
	id: u32,
	ore_ore: u32,
	clay_ore: u32,
	obsidian_ore: u32,
	obsidian_clay: u32,
	geode_ore: u32,
	geode_obsidian: u32,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct MiningState {
	ore: u32,
	clay: u32,
	obsidian: u32,
	geodes: u32,
	ore_robots: u32,
	clay_robots: u32,
	obsidian_robots: u32,
	geode_robots: u32,
}

impl Default for MiningState {
	fn default() -> Self {
		Self {
			ore: 0,
			clay: 0,
			obsidian: 0,
			geodes: 0,
			ore_robots: 1,
			clay_robots: 0,
			obsidian_robots: 0,
			geode_robots: 0,
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let blueprints: Vec<Blueprint> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut blueprints = Vec::new();

		for line in input_string.lines().filter(|s| !s.is_empty()) {
			let line_data = line.strip_prefix("Blueprint ").unwrap();
			let mut id = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ':' {
					break;
				}
				id.push(c);
			}
			let id: u32 = id.parse()?;
			let line_data: String = line_chars.collect();

			let line_data = line_data.strip_prefix(" Each ore robot costs ").unwrap();
			let mut ore_ore = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ' ' {
					break;
				}
				ore_ore.push(c);
			}
			let ore_ore: u32 = ore_ore.parse()?;
			let line_data: String = line_chars.collect();

			let line_data = line_data.strip_prefix("ore. Each clay robot costs ").unwrap();
			let mut clay_ore = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ' ' {
					break;
				}
				clay_ore.push(c);
			}
			let clay_ore: u32 = clay_ore.parse()?;
			let line_data: String = line_chars.collect();

			let line_data = line_data.strip_prefix("ore. Each obsidian robot costs ").unwrap();
			let mut obsidian_ore = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ' ' {
					break;
				}
				obsidian_ore.push(c);
			}
			let obsidian_ore: u32 = obsidian_ore.parse()?;
			let line_data: String = line_chars.collect();

			let line_data = line_data.strip_prefix("ore and ").unwrap();
			let mut obsidian_clay = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ' ' {
					break;
				}
				obsidian_clay.push(c);
			}
			let obsidian_clay: u32 = obsidian_clay.parse()?;
			let line_data: String = line_chars.collect();

			let line_data = line_data.strip_prefix("clay. Each geode robot costs ").unwrap();
			let mut geode_ore = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ' ' {
					break;
				}
				geode_ore.push(c);
			}
			let geode_ore: u32 = geode_ore.parse()?;
			let line_data: String = line_chars.collect();

			let line_data = line_data.strip_prefix("ore and ").unwrap();
			let mut geode_obsidian = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ' ' {
					break;
				}
				geode_obsidian.push(c);
			}
			let geode_obsidian: u32 = geode_obsidian.parse()?;
			let line_data: String = line_chars.collect();

			assert_eq!(line_data, "obsidian.");

			let blueprint = Blueprint {
				id,
				ore_ore,
				clay_ore,
				obsidian_ore,
				obsidian_clay,
				geode_ore,
				geode_obsidian,
			};
			blueprints.push(blueprint);
		}

		blueprints
	};

	let mut total_quality_level = 0;
	for blueprint in blueprints.iter() {
		let mut states: HashSet<MiningState> = HashSet::new();
		states.insert(MiningState::default());
		for _ in 0..MAX_TIME {
			let mut new_states = HashSet::new();
			for state in states.iter() {
				let can_build_ore_robot = state.ore >= blueprint.ore_ore;
				let can_build_clay_robot = state.ore >= blueprint.clay_ore;
				let can_build_obsidian_robot =
					state.ore >= blueprint.obsidian_ore && state.clay >= blueprint.obsidian_clay;
				let can_build_geode_robot =
					state.ore >= blueprint.geode_ore && state.obsidian >= blueprint.geode_obsidian;

				let mut new_state = state.clone();
				new_state.ore += state.ore_robots;
				new_state.clay += state.clay_robots;
				new_state.obsidian += state.obsidian_robots;
				new_state.geodes += state.geode_robots;

				if can_build_ore_robot {
					let mut new_robot_state = new_state.clone();
					new_robot_state.ore_robots += 1;
					new_robot_state.ore -= blueprint.ore_ore;
					new_states.insert(new_robot_state);
				}
				if can_build_clay_robot {
					let mut new_robot_state = new_state.clone();
					new_robot_state.clay_robots += 1;
					new_robot_state.ore -= blueprint.clay_ore;
					new_states.insert(new_robot_state);
				}
				if can_build_obsidian_robot {
					let mut new_robot_state = new_state.clone();
					new_robot_state.obsidian_robots += 1;
					new_robot_state.ore -= blueprint.obsidian_ore;
					new_robot_state.clay -= blueprint.obsidian_clay;
					new_states.insert(new_robot_state);
				}
				if can_build_geode_robot {
					let mut new_robot_state = new_state.clone();
					new_robot_state.geode_robots += 1;
					new_robot_state.ore -= blueprint.geode_ore;
					new_robot_state.obsidian -= blueprint.geode_obsidian;
					new_states.insert(new_robot_state);
				}
				new_states.insert(new_state);
			}
			states = new_states;
		}
		let most_geodes = states.iter().map(|state| state.geodes).max().unwrap();
		let quality_level = most_geodes * blueprint.id;
		total_quality_level += quality_level;
	}

	println!("{}", total_quality_level);

	Ok(())
}
