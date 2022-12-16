use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fs;

struct ValveData {
	flow_rate: u32,
	to_valves: Vec<String>,
}

#[derive(Clone, Eq, PartialEq)]
struct ValveOpenProgress {
	released_pressure: u32,
	minutes_passed: u32,
	open_valves: HashSet<String>,
	current_valve: String,
	visited_at_current_open_valve_combo: HashSet<String>,
}

impl Default for ValveOpenProgress {
	fn default() -> Self {
		Self {
			released_pressure: 0,
			minutes_passed: 0,
			open_valves: HashSet::new(),
			current_valve: String::from("AA"),
			visited_at_current_open_valve_combo: HashSet::new(),
		}
	}
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

	let initial_progress = ValveOpenProgress::default();
	let mut open_progress: BinaryHeap<ValveOpenProgress> = BinaryHeap::new();
	open_progress.push(initial_progress);
	let valves_that_work: HashSet<String> = valves
		.iter()
		.filter(|(_, data)| data.flow_rate > 0)
		.map(|(name, _)| name.clone())
		.collect();

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
		let minute = progress.minutes_passed + 1;
		if minute == 30 {
			if progress.released_pressure > max_released_pressure {
				println!("{} {:?}", progress.released_pressure, progress.open_valves);
				max_released_pressure = progress.released_pressure;
			}
			continue;
		}
		progress.minutes_passed = minute;
		progress
			.visited_at_current_open_valve_combo
			.insert(progress.current_valve.clone());

		let current_valve = valves.get(&progress.current_valve).unwrap();

		if !progress.open_valves.contains(&progress.current_valve) && current_valve.flow_rate > 0 {
			let mut next_progress = progress.clone();
			next_progress.open_valves.insert(progress.current_valve.clone());
			next_progress.released_pressure +=
				valves.get(&progress.current_valve).unwrap().flow_rate * (30 - progress.minutes_passed);
			next_progress.visited_at_current_open_valve_combo.clear();
			open_progress.push(next_progress);
		}

		for next_valve in valves
			.get(&progress.current_valve)
			.unwrap()
			.to_valves
			.iter()
			.filter(|v| !progress.visited_at_current_open_valve_combo.contains(*v))
		{
			let mut next_progress = progress.clone();
			next_progress.current_valve = next_valve.clone();
			open_progress.push(next_progress);
		}
	}

	println!("{}", max_released_pressure);

	Ok(())
}
