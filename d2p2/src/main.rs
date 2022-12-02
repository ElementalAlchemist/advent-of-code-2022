use std::error::Error;
use std::fs;

#[derive(Clone)]
enum Selection {
	Rock,
	Paper,
	Scissors,
}

impl Selection {
	fn base_score(&self) -> u32 {
		match self {
			Self::Rock => 1,
			Self::Paper => 2,
			Self::Scissors => 3,
		}
	}

	fn win_against(&self) -> Self {
		match self {
			Self::Rock => Self::Paper,
			Self::Paper => Self::Scissors,
			Self::Scissors => Self::Rock,
		}
	}

	fn lose_against(&self) -> Self {
		match self {
			Self::Rock => Self::Scissors,
			Self::Paper => Self::Rock,
			Self::Scissors => Self::Paper,
		}
	}
}

enum Outcome {
	Win,
	Loss,
	Draw,
}

impl Outcome {
	fn score(&self) -> u32 {
		match self {
			Self::Win => 6,
			Self::Draw => 3,
			Self::Loss => 0,
		}
	}
}

struct RoundResult {
	selection: Selection,
	outcome: Outcome,
}

impl RoundResult {
	fn score(&self) -> u32 {
		self.selection.base_score() + self.outcome.score()
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let rounds: Vec<RoundResult> = {
		let input = fs::read_to_string("input.txt")?;

		let mut rounds: Vec<RoundResult> = Vec::new();
		for input_line in input.lines().filter(|s| !s.is_empty()) {
			let mut moves = input_line.split(' ');
			let opponent_move = moves.next().unwrap();
			let my_move = moves.next().unwrap();
			let opponent_move = match opponent_move {
				"A" => Selection::Rock,
				"B" => Selection::Paper,
				"C" => Selection::Scissors,
				_ => unreachable!(),
			};
			let (my_move, outcome) = match my_move {
				"X" => (opponent_move.lose_against(), Outcome::Loss),
				"Y" => (opponent_move.clone(), Outcome::Draw),
				"Z" => (opponent_move.win_against(), Outcome::Win),
				_ => unreachable!(),
			};
			rounds.push(RoundResult {
				selection: my_move,
				outcome,
			});
		}

		rounds
	};

	let score: u32 = rounds.iter().map(|r| r.score()).sum();
	println!("{}", score);

	Ok(())
}
