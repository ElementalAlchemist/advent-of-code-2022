use std::collections::HashMap;
use std::error::Error;
use std::fs;

const TOTAL_FILESYSTEM_SIZE: u32 = 70_000_000;
const SPACE_REQUIRED_FOR_UPDATE: u32 = 30_000_000;

#[derive(Clone)]
struct File {
	name: String,
	size: u32,
}

#[derive(Default)]
struct Directory {
	files: Vec<File>,
	subdirectories: HashMap<String, Directory>,
}

impl Directory {
	fn total_size(&self) -> u32 {
		let file_size: u32 = self.files.iter().map(|f| f.size).sum();
		let dir_size: u32 = self.subdirectories.values().map(|d| d.total_size()).sum();
		file_size + dir_size
	}
}

enum Command {
	ChangeDirectory(String),
	List,
}

enum OutputLine {
	File(File),
	Directory(String),
}

enum Line {
	Input(Command),
	Output(OutputLine),
}

fn main() -> Result<(), Box<dyn Error>> {
	let data = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut data: Vec<Line> = Vec::new();
		for line in input_string.lines().filter(|s| !s.is_empty()) {
			if line == "$ ls" {
				data.push(Line::Input(Command::List));
			} else if let Some(dir) = line.strip_prefix("$ cd ") {
				data.push(Line::Input(Command::ChangeDirectory(String::from(dir))));
			} else if let Some(dir) = line.strip_prefix("dir ") {
				data.push(Line::Output(OutputLine::Directory(String::from(dir))));
			} else {
				let mut parts = line.split(' ');
				let size: u32 = parts.next().unwrap().parse().unwrap();
				let name = parts.next().unwrap();
				data.push(Line::Output(OutputLine::File(File {
					name: String::from(name),
					size,
				})));
			}
		}

		data
	};

	let mut filesystem = Directory::default();
	let mut current_directory: Vec<String> = Vec::new();

	for line in data.iter() {
		match line {
			Line::Input(input) => match input {
				Command::ChangeDirectory(dir_path) => {
					let mut dir_path = dir_path.clone();
					if let Some(path) = dir_path.strip_prefix('/') {
						current_directory.clear();
						dir_path = String::from(path);
					}
					if !dir_path.is_empty() {
						for component in dir_path.split('/') {
							if component == ".." {
								current_directory.pop();
							} else {
								current_directory.push(String::from(component));
							}
						}
					}
				}
				Command::List => (),
			},
			Line::Output(output) => match output {
				OutputLine::File(file_data) => {
					let mut current_dir = &mut filesystem;
					for component in current_directory.iter() {
						current_dir = current_dir.subdirectories.get_mut(component).unwrap();
					}
					current_dir.files.push(file_data.clone());
				}
				OutputLine::Directory(dir_name) => {
					let mut current_dir = &mut filesystem;
					for component in current_directory.iter() {
						current_dir = current_dir.subdirectories.get_mut(component).unwrap();
					}
					current_dir
						.subdirectories
						.insert(dir_name.clone(), Directory::default());
				}
			},
		}
	}

	let mut check_locations: Vec<&Directory> = vec![&filesystem];
	let mut dir_sizes: Vec<u32> = Vec::new();
	while let Some(location) = check_locations.pop() {
		dir_sizes.push(location.total_size());
		for next_loc in location.subdirectories.values() {
			check_locations.push(next_loc);
		}
	}

	dir_sizes.sort_unstable();

	let used_space = dir_sizes.last().unwrap();
	let unused_space = TOTAL_FILESYSTEM_SIZE - used_space;
	let required_space = SPACE_REQUIRED_FOR_UPDATE - unused_space;

	for size in dir_sizes.iter() {
		if *size >= required_space {
			println!("{}", *size);
			break;
		}
	}

	Ok(())
}
