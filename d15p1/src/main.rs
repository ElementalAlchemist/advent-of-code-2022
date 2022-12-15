use std::collections::HashSet;
use std::error::Error;
use std::fs;

const TARGET_Y_LEVEL: i32 = 2000000;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

struct Sensor {
	position: Coordinate,
	closest_beacon: Coordinate,
}

impl Sensor {
	fn beacon_distance(&self) -> u32 {
		self.position.x.abs_diff(self.closest_beacon.x) + self.position.y.abs_diff(self.closest_beacon.y)
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let sensors: Vec<Sensor> = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut sensors = Vec::new();
		for line in input_string.lines().filter(|s| !s.is_empty()) {
			let line_data = line.strip_prefix("Sensor at x=").unwrap();
			let mut sensor_x = String::new();
			let mut line_chars = line_data.chars();
			for val in line_chars.by_ref() {
				if val == ',' {
					break;
				}
				sensor_x.push(val);
			}
			let sensor_x: i32 = sensor_x.parse()?;

			let line_data: String = line_chars.collect();
			let line_data = line_data.strip_prefix(" y=").unwrap();
			let mut line_chars = line_data.chars();
			let mut sensor_y = String::new();
			for val in line_chars.by_ref() {
				if val == ':' {
					break;
				}
				sensor_y.push(val);
			}
			let sensor_y: i32 = sensor_y.parse()?;

			let line_data: String = line_chars.collect();
			let line_data = line_data.strip_prefix(" closest beacon is at x=").unwrap();
			let mut beacon_x = String::new();
			let mut line_chars = line_data.chars();
			for val in line_chars.by_ref() {
				if val == ',' {
					break;
				}
				beacon_x.push(val);
			}
			let beacon_x: i32 = beacon_x.parse()?;

			let line_data: String = line_chars.collect();
			let line_data = line_data.strip_prefix(" y=").unwrap();
			let beacon_y: i32 = line_data.parse()?;

			let position = Coordinate {
				x: sensor_x,
				y: sensor_y,
			};
			let closest_beacon = Coordinate {
				x: beacon_x,
				y: beacon_y,
			};

			sensors.push(Sensor {
				position,
				closest_beacon,
			});
		}

		sensors
	};

	let beacons: HashSet<Coordinate> = sensors.iter().map(|sensor| sensor.closest_beacon).collect();
	let mut cannot_contain: HashSet<i32> = HashSet::new();
	for sensor in sensors.iter() {
		let sensor_distance = sensor.beacon_distance();
		let vertical_distance = sensor.position.y.abs_diff(TARGET_Y_LEVEL);
		let remaining_distance = sensor_distance.saturating_sub(vertical_distance);
		let remaining_distance: i32 = remaining_distance.try_into().unwrap();
		for x in (sensor.position.x - remaining_distance)..=(sensor.position.x + remaining_distance) {
			if beacons.contains(&Coordinate { x, y: TARGET_Y_LEVEL }) {
				continue;
			}
			cannot_contain.insert(x);
		}
	}
	println!("{}", cannot_contain.len());

	Ok(())
}
