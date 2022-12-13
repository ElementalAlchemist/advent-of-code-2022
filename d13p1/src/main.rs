use std::cmp::Ordering;
use std::error::Error;
use std::fs;

#[derive(Clone)]
enum PacketDatum {
	List(Vec<PacketDatum>),
	Integer(i32),
}

fn packet_data_in_order(left: &PacketDatum, right: &PacketDatum) -> Option<bool> {
	match (left, right) {
		(PacketDatum::Integer(first), PacketDatum::Integer(second)) => match first.cmp(second) {
			Ordering::Less => Some(true),
			Ordering::Greater => Some(false),
			Ordering::Equal => None,
		},
		(PacketDatum::List(first), PacketDatum::List(second)) => {
			for index in 0..(first.len().max(second.len())) {
				if index >= first.len() {
					return Some(true);
				}
				if index >= second.len() {
					return Some(false);
				}
				if let Some(val) = packet_data_in_order(&first[index], &second[index]) {
					return Some(val);
				}
			}
			None
		}
		(PacketDatum::Integer(first), PacketDatum::List(_)) => {
			packet_data_in_order(&PacketDatum::List(vec![PacketDatum::Integer(*first)]), right)
		}
		(PacketDatum::List(_), PacketDatum::Integer(second)) => {
			packet_data_in_order(left, &PacketDatum::List(vec![PacketDatum::Integer(*second)]))
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let packets: Vec<(Vec<PacketDatum>, Vec<PacketDatum>)> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut packets = Vec::new();

		let mut first_packet_list_parse: Vec<Vec<PacketDatum>> = vec![Vec::new()];
		let mut second_packet_list_parse: Vec<Vec<PacketDatum>> = vec![Vec::new()];
		let mut parsing_second = false;
		for packet_str in input_string.lines().filter(|s| !s.is_empty()) {
			let Some(packet_str) = packet_str.strip_prefix('[') else {
				panic!("Invalid packet format");
			};
			let Some(packet_str) = packet_str.strip_suffix(']') else {
				panic!("Invalid packet format");
			};

			for entry in packet_str.split(',') {
				let mut datum = entry;
				while let Some(datum_stripped) = datum.strip_prefix('[') {
					if parsing_second {
						second_packet_list_parse.push(Vec::new());
					} else {
						first_packet_list_parse.push(Vec::new());
					}
					datum = datum_stripped;
				}

				let mut resolve_levels: u32 = 0;
				while let Some(datum_stripped) = datum.strip_suffix(']') {
					resolve_levels += 1;
					datum = datum_stripped;
				}

				if !datum.is_empty() {
					if parsing_second {
						second_packet_list_parse
							.last_mut()
							.unwrap()
							.push(PacketDatum::Integer(datum.parse()?));
					} else {
						first_packet_list_parse
							.last_mut()
							.unwrap()
							.push(PacketDatum::Integer(datum.parse()?));
					}
				}

				for _ in 0..resolve_levels {
					if parsing_second {
						let resolve_list = second_packet_list_parse.pop().unwrap();
						second_packet_list_parse
							.last_mut()
							.unwrap()
							.push(PacketDatum::List(resolve_list));
					} else {
						let resolve_list = first_packet_list_parse.pop().unwrap();
						first_packet_list_parse
							.last_mut()
							.unwrap()
							.push(PacketDatum::List(resolve_list));
					}
				}
			}

			if parsing_second {
				parsing_second = false;
				assert!(
					first_packet_list_parse.len() == 1 && second_packet_list_parse.len() == 1,
					"All lists resolved correctly"
				);
				packets.push((first_packet_list_parse[0].clone(), second_packet_list_parse[0].clone()));
				first_packet_list_parse = vec![Vec::new()];
				second_packet_list_parse = vec![Vec::new()];
			} else {
				parsing_second = true;
			}
		}
		packets.push((first_packet_list_parse[0].clone(), second_packet_list_parse[0].clone()));

		packets
	};

	let mut index_sum = 0;
	for (packet_index, (first_packet, second_packet)) in packets.iter().enumerate() {
		for index in 0..(first_packet.len().max(second_packet.len())) {
			if index >= first_packet.len() {
				index_sum += packet_index + 1;
				break;
			}
			if index >= second_packet.len() {
				break;
			}
			if let Some(in_order) = packet_data_in_order(&first_packet[index], &second_packet[index]) {
				if in_order {
					index_sum += packet_index + 1;
				}
				break;
			}
		}
	}

	println!("{}", index_sum);

	Ok(())
}
