use std::collections::BinaryHeap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input_string = fs::read_to_string("input.txt")?;
	let mut elf_totals: BinaryHeap<i32> = BinaryHeap::new();
	let mut elf_total = 0;
	for val in input_string.split('\n') {
		if val.is_empty() {
			elf_totals.push(elf_total);
			elf_total = 0;
			continue;
		}
		let cal_val: i32 = val.parse()?;
		elf_total += cal_val;
	}
	elf_totals.push(elf_total);

	let biggest = elf_totals.pop().unwrap();
	println!("{}", biggest);

	Ok(())
}
