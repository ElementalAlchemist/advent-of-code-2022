use std::error::Error;
use std::fs;

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

fn match_result(my_move: &Selection, opponent_move: &Selection) -> Outcome {
	match (my_move, opponent_move) {
		(Selection::Rock, Selection::Rock) => Outcome::Draw,
		(Selection::Rock, Selection::Paper) => Outcome::Loss,
		(Selection::Rock, Selection::Scissors) => Outcome::Win,
		(Selection::Paper, Selection::Rock) => Outcome::Win,
		(Selection::Paper, Selection::Paper) => Outcome::Draw,
		(Selection::Paper, Selection::Scissors) => Outcome::Loss,
		(Selection::Scissors, Selection::Rock) => Outcome::Loss,
		(Selection::Scissors, Selection::Paper) => Outcome::Win,
		(Selection::Scissors, Selection::Scissors) => Outcome::Draw,
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
			let my_move = match my_move {
				"X" => Selection::Rock,
				"Y" => Selection::Paper,
				"Z" => Selection::Scissors,
				_ => unreachable!(),
			};
			let outcome = match_result(&my_move, &opponent_move);
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
