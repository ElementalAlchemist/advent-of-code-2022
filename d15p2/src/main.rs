use std::error::Error;
use std::fs;

const MAX_COORDINATE: i64 = 4000000;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i64,
	y: i64,
}

struct Sensor {
	position: Coordinate,
	closest_beacon: Coordinate,
}

impl Sensor {
	fn beacon_distance(&self) -> u64 {
		self.position.x.abs_diff(self.closest_beacon.x) + self.position.y.abs_diff(self.closest_beacon.y)
	}

	fn outside_border_coordinates(&self) -> Vec<Coordinate> {
		let sensor_distance: i64 = self.beacon_distance().try_into().unwrap();
		let sensor_distance = sensor_distance + 1;
		let top = Coordinate {
			x: self.position.x,
			y: self.position.y + sensor_distance,
		};
		let mut border_coordinates: Vec<Coordinate> = Vec::new();
		let mut current = top;
		let right = Coordinate {
			x: self.position.x + sensor_distance,
			y: self.position.y,
		};
		while current != right {
			current.x += 1;
			current.y -= 1;
			border_coordinates.push(current);
		}
		let bottom = Coordinate {
			x: self.position.x,
			y: self.position.y - sensor_distance,
		};
		while current != bottom {
			current.x -= 1;
			current.y -= 1;
			border_coordinates.push(current);
		}
		let left = Coordinate {
			x: self.position.x - sensor_distance,
			y: self.position.y,
		};
		while current != left {
			current.x -= 1;
			current.y += 1;
			border_coordinates.push(current);
		}
		while current != top {
			current.x += 1;
			current.y += 1;
			border_coordinates.push(current);
		}
		border_coordinates
	}

	fn can_sense_coordinate(&self, coord: &Coordinate) -> bool {
		self.position.x.abs_diff(coord.x) + self.position.y.abs_diff(coord.y) <= self.beacon_distance()
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
			let sensor_x: i64 = sensor_x.parse()?;

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
			let sensor_y: i64 = sensor_y.parse()?;

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
			let beacon_x: i64 = beacon_x.parse()?;

			let line_data: String = line_chars.collect();
			let line_data = line_data.strip_prefix(" y=").unwrap();
			let beacon_y: i64 = line_data.parse()?;

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

	for sensor in sensors.iter() {
		let sensor_border_coordinates = sensor.outside_border_coordinates();
		for coordinate in sensor_border_coordinates
			.iter()
			.filter(|coord| coord.x >= 0 && coord.x <= MAX_COORDINATE && coord.y >= 0 && coord.y <= MAX_COORDINATE)
		{
			if !sensors.iter().any(|s| s.can_sense_coordinate(coordinate)) {
				let tuning_frequency = coordinate.x * 4000000 + coordinate.y;
				println!("({}, {}) => {}", coordinate.x, coordinate.y, tuning_frequency);
			}
		}
	}

	Ok(())
}
