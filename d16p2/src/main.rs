use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fs;

struct ValveData {
	flow_rate: u32,
	to_valves: Vec<String>,
}

#[derive(Clone, Eq, PartialEq)]
enum ValveLocation {
	AtValve(String),
	GoingToValve(String, u32),
}

#[derive(Clone, Eq, PartialEq)]
struct ValveOpenProgress {
	released_pressure: u32,
	minutes_passed: u32,
	open_valves: HashSet<String>,
	current_valve: ValveLocation,
	elephant_valve: ValveLocation,
}

impl Ord for ValveOpenProgress {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.released_pressure
			.cmp(&other.released_pressure)
			.then_with(|| self.minutes_passed.cmp(&other.minutes_passed))
			.then_with(|| self.open_valves.len().cmp(&other.open_valves.len()))
	}
}

impl PartialOrd for ValveOpenProgress {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

struct TrackValveProgress {
	current_valve: String,
	visited: HashSet<String>,
}

impl TrackValveProgress {
	fn new(start_valve: &str) -> Self {
		let mut visited = HashSet::new();
		visited.insert(String::from(start_valve));
		Self {
			current_valve: String::from(start_valve),
			visited,
		}
	}
}

struct ValveWithDistance {
	valve: String,
	distance: u32,
}

fn build_valve_map_from_valve(
	start_valve: &str,
	valves_that_work: &HashSet<String>,
	valves: &HashMap<String, ValveData>,
	valve_map: &mut HashMap<String, Vec<ValveWithDistance>>,
) {
	let mut distance = 0;
	let mut at_valves = vec![TrackValveProgress::new(start_valve)];
	while !at_valves.is_empty() {
		let mut new_at_valves = Vec::new();
		for valve in at_valves.drain(..) {
			for next_valve in valves
				.get(&valve.current_valve)
				.unwrap()
				.to_valves
				.iter()
				.filter(|v| !valve.visited.contains(*v))
			{
				let mut next_valve_track = TrackValveProgress {
					current_valve: next_valve.clone(),
					visited: valve.visited.clone(),
				};
				next_valve_track.visited.insert(next_valve_track.current_valve.clone());
				new_at_valves.push(next_valve_track);
			}
			let valve_map_entry = valve_map.entry(String::from(start_valve)).or_default();
			if valve.current_valve != *start_valve
				&& !valve_map_entry.iter().any(|ent| ent.valve == valve.current_valve)
				&& valves_that_work.contains(&valve.current_valve)
			{
				let valve_data = ValveWithDistance {
					valve: valve.current_valve.clone(),
					distance,
				};
				valve_map_entry.push(valve_data);
			}
		}
		distance += 1;
		at_valves = new_at_valves;
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let valves: HashMap<String, ValveData> = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut valves = HashMap::new();

		for line in input_string.lines().filter(|s| !s.is_empty()) {
			let line_data = line.strip_prefix("Valve ").unwrap();
			let mut valve_name = String::new();
			let mut line_chars = line_data.chars();
			for c in line_chars.by_ref() {
				if c == ' ' {
					break;
				}
				valve_name.push(c);
			}
			let line_data: String = line_chars.collect();
			let line_data = line_data.strip_prefix("has flow rate=").unwrap();
			let mut parts = line_data.split(';');
			let flow_rate = parts.next().unwrap();
			let line_data = parts.next().unwrap();
			assert!(parts.next().is_none());
			let flow_rate: u32 = flow_rate.parse()?;
			let line_data = if let Some(data) = line_data.strip_prefix(" tunnels lead to valves ") {
				data
			} else if let Some(data) = line_data.strip_prefix(" tunnel leads to valve ") {
				data
			} else {
				panic!("Unexpected line data leading to valves: {}", line_data);
			};
			let to_valves: Vec<String> = line_data.split(", ").map(String::from).collect();
			let valve_data = ValveData { flow_rate, to_valves };
			valves.insert(valve_name, valve_data);
		}

		valves
	};

	let valves_that_work: HashSet<String> = valves
		.iter()
		.filter(|(_, data)| data.flow_rate > 0)
		.map(|(name, _)| name.clone())
		.collect();
	let mut valve_map: HashMap<String, Vec<ValveWithDistance>> = HashMap::new();
	build_valve_map_from_valve("AA", &valves_that_work, &valves, &mut valve_map);
	for valve in valves_that_work.iter() {
		build_valve_map_from_valve(valve, &valves_that_work, &valves, &mut valve_map);
	}

	let mut open_progress: BinaryHeap<ValveOpenProgress> = BinaryHeap::new();
	for start_valve in valve_map.get("AA").unwrap().iter() {
		for elephant_valve in valve_map.get("AA").unwrap().iter() {
			let start_progress = ValveOpenProgress {
				released_pressure: 0,
				minutes_passed: 0,
				open_valves: HashSet::new(),
				current_valve: ValveLocation::GoingToValve(start_valve.valve.clone(), start_valve.distance),
				elephant_valve: ValveLocation::GoingToValve(elephant_valve.valve.clone(), elephant_valve.distance),
			};
			open_progress.push(start_progress);
		}
	}

	let mut max_released_pressure = 0;
	while let Some(mut progress) = open_progress.pop() {
		if progress.open_valves == valves_that_work {
			if progress.released_pressure > max_released_pressure {
				println!(
					"{} {:?} [{} minutes]",
					progress.released_pressure, progress.open_valves, progress.minutes_passed
				);
				max_released_pressure = progress.released_pressure;
			}
			continue;
		}
		if progress.minutes_passed >= 26 {
			if progress.released_pressure > max_released_pressure {
				println!("{} {:?}", progress.released_pressure, progress.open_valves);
				max_released_pressure = progress.released_pressure;
			}
			continue;
		}

		match (&progress.current_valve, &progress.elephant_valve) {
			(ValveLocation::AtValve(current_valve), ValveLocation::AtValve(elephant_valve)) => {
				progress.minutes_passed += 1;
				if progress.open_valves.insert(current_valve.clone()) {
					progress.released_pressure +=
						valves.get(current_valve).unwrap().flow_rate * (26 - progress.minutes_passed);
				}
				if progress.open_valves.insert(elephant_valve.clone()) {
					progress.released_pressure +=
						valves.get(elephant_valve).unwrap().flow_rate * (26 - progress.minutes_passed);
				}

				let mut next_current_valves = Vec::new();
				for next_valve in valve_map
					.get(current_valve)
					.unwrap()
					.iter()
					.filter(|v| !progress.open_valves.contains(&v.valve))
				{
					let mut next_progress = progress.clone();
					next_progress.current_valve =
						ValveLocation::GoingToValve(next_valve.valve.clone(), next_valve.distance);
					next_current_valves.push(next_progress);
				}
				let mut next_elephant_valves = Vec::new();
				for next_progress in next_current_valves.drain(..) {
					for next_valve in valve_map
						.get(elephant_valve)
						.unwrap()
						.iter()
						.filter(|v| !next_progress.open_valves.contains(&v.valve))
					{
						let mut next_progress = next_progress.clone();
						next_progress.elephant_valve =
							ValveLocation::GoingToValve(next_valve.valve.clone(), next_valve.distance);
						next_elephant_valves.push(next_progress);
					}
				}
				if next_elephant_valves.is_empty() {
					open_progress.push(progress);
				} else {
					for valve_progress in next_elephant_valves.drain(..) {
						open_progress.push(valve_progress);
					}
				}
			}
			(
				ValveLocation::GoingToValve(current_valve, current_valve_distance),
				ValveLocation::AtValve(elephant_valve),
			) => {
				progress.minutes_passed += 1;
				let next_current_valve = if *current_valve_distance == 1 {
					ValveLocation::AtValve(current_valve.clone())
				} else {
					ValveLocation::GoingToValve(current_valve.clone(), *current_valve_distance - 1)
				};
				if progress.open_valves.insert(elephant_valve.clone()) {
					progress.released_pressure +=
						valves.get(elephant_valve).unwrap().flow_rate * (26 - progress.minutes_passed);
				}

				let mut added_progress = false;
				for next_valve in valve_map
					.get(elephant_valve)
					.unwrap()
					.iter()
					.filter(|v| !progress.open_valves.contains(&v.valve) && v.valve != *current_valve)
				{
					let mut next_progress = progress.clone();
					next_progress.elephant_valve =
						ValveLocation::GoingToValve(next_valve.valve.clone(), next_valve.distance);
					next_progress.current_valve = next_current_valve.clone();
					open_progress.push(next_progress);
					added_progress = true;
				}
				if !added_progress {
					let mut next_progress = progress.clone();
					next_progress.current_valve = next_current_valve;
					open_progress.push(next_progress);
				}
			}
			(
				ValveLocation::AtValve(current_valve),
				ValveLocation::GoingToValve(elephant_valve, elephant_valve_distance),
			) => {
				progress.minutes_passed += 1;
				let next_elephant_valve = if *elephant_valve_distance == 1 {
					ValveLocation::AtValve(elephant_valve.clone())
				} else {
					ValveLocation::GoingToValve(elephant_valve.clone(), *elephant_valve_distance - 1)
				};
				if progress.open_valves.insert(current_valve.clone()) {
					progress.released_pressure +=
						valves.get(current_valve).unwrap().flow_rate * (26 - progress.minutes_passed);
				}

				let mut added_progress = false;
				for next_valve in valve_map
					.get(current_valve)
					.unwrap()
					.iter()
					.filter(|v| !progress.open_valves.contains(&v.valve) && v.valve != *elephant_valve)
				{
					let mut next_progress = progress.clone();
					next_progress.current_valve =
						ValveLocation::GoingToValve(next_valve.valve.clone(), next_valve.distance);
					next_progress.elephant_valve = next_elephant_valve.clone();
					open_progress.push(next_progress);
					added_progress = true;
				}
				if !added_progress {
					let mut next_progress = progress.clone();
					next_progress.elephant_valve = next_elephant_valve;
					open_progress.push(next_progress);
				}
			}
			(
				ValveLocation::GoingToValve(current_valve, current_valve_distance),
				ValveLocation::GoingToValve(elephant_valve, elephant_valve_distance),
			) => {
				let pass_minutes = (*current_valve_distance).min(*elephant_valve_distance);
				progress.minutes_passed += pass_minutes;
				let new_current_distance = *current_valve_distance - pass_minutes;
				let new_elephant_distance = *elephant_valve_distance - pass_minutes;

				let mut next_progress = progress.clone();
				next_progress.current_valve = if new_current_distance == 0 {
					ValveLocation::AtValve(current_valve.clone())
				} else {
					ValveLocation::GoingToValve(current_valve.clone(), new_current_distance)
				};
				next_progress.elephant_valve = if new_elephant_distance == 0 {
					ValveLocation::AtValve(elephant_valve.clone())
				} else {
					ValveLocation::GoingToValve(elephant_valve.clone(), new_elephant_distance)
				};
				open_progress.push(next_progress);
			}
		}
	}

	println!("{}", max_released_pressure);

	Ok(())
}
