use std::cmp::Ordering;
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, PartialEq)]
enum PacketDatum {
	List(Vec<PacketDatum>),
	Integer(i32),
}

impl Ord for PacketDatum {
	fn cmp(&self, other: &Self) -> Ordering {
		packet_data_in_order(self, other)
	}
}

impl PartialOrd for PacketDatum {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn packet_data_in_order(left: &PacketDatum, right: &PacketDatum) -> Ordering {
	match (left, right) {
		(PacketDatum::Integer(first), PacketDatum::Integer(second)) => first.cmp(second),
		(PacketDatum::List(first), PacketDatum::List(second)) => {
			for index in 0..(first.len().max(second.len())) {
				if index >= first.len() {
					return Ordering::Less;
				}
				if index >= second.len() {
					return Ordering::Greater;
				}
				let val = packet_data_in_order(&first[index], &second[index]);
				if val != Ordering::Equal {
					return val;
				}
			}
			Ordering::Equal
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
	let packets: Vec<Vec<PacketDatum>> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut packets = Vec::new();

		let mut packet_list_parse: Vec<Vec<PacketDatum>> = vec![Vec::new()];
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
					packet_list_parse.push(Vec::new());
					datum = datum_stripped;
				}

				let mut resolve_levels: u32 = 0;
				while let Some(datum_stripped) = datum.strip_suffix(']') {
					resolve_levels += 1;
					datum = datum_stripped;
				}

				if !datum.is_empty() {
					packet_list_parse
						.last_mut()
						.unwrap()
						.push(PacketDatum::Integer(datum.parse()?));
				}

				for _ in 0..resolve_levels {
					let resolve_list = packet_list_parse.pop().unwrap();
					packet_list_parse
						.last_mut()
						.unwrap()
						.push(PacketDatum::List(resolve_list));
				}
			}

			assert!(packet_list_parse.len() == 1, "All lists resolved correctly");
			packets.push(packet_list_parse[0].clone());
			packet_list_parse = vec![Vec::new()];
		}
		packets.push(packet_list_parse[0].clone());

		packets.push(vec![PacketDatum::List(vec![PacketDatum::Integer(2)])]);
		packets.push(vec![PacketDatum::List(vec![PacketDatum::Integer(6)])]);

		packets.sort_unstable();

		packets
	};

	let mut marker_2_index: Option<usize> = None;
	let mut marker_6_index: Option<usize> = None;

	for (packet_index, packet) in packets.iter().enumerate() {
		if packet.len() == 1 {
			if let PacketDatum::List(packet_list) = &packet[0] {
				if packet_list.len() == 1 {
					if let PacketDatum::Integer(packet_int) = &packet_list[0] {
						if *packet_int == 2 {
							marker_2_index = Some(packet_index);
						} else if *packet_int == 6 {
							marker_6_index = Some(packet_index);
						}
					}
				}
			}
		}
	}

	let marker_2_index = marker_2_index.unwrap();
	let marker_6_index = marker_6_index.unwrap();
	let index_product = marker_2_index * marker_6_index;
	println!("{}", index_product);

	Ok(())
}
