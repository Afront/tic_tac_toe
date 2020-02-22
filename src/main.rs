use promptly::prompt;

#[derive(Debug, PartialEq)]
enum State {
	Blank,
	FirstPlayer,
	SecondPlayer,
}

impl State {
	fn as_uint16(&self) -> u16 {
		match self {
			State::FirstPlayer => 1,
			State::SecondPlayer => 2,
			State::Blank => 0,
		}
	}

	fn next(&self) -> Self {
		match self {
			State::FirstPlayer => State::SecondPlayer,
			State::SecondPlayer => State::FirstPlayer,
			State::Blank => State::Blank,
		}
	}
}

/**
 * Possibilities for grid:
 * A 2D array: size = 9
 * A 1D array: size = 9
 * An int16: size = 2
 *
 */

#[derive(Debug)]
struct TicTacToe {
	grid: u16,
}

impl Default for TicTacToe {
	fn default() -> TicTacToe {
		TicTacToe { grid: 0 }
	}
}

impl TicTacToe {
	fn print(&self) {
		let symbols = ["_", "O", "X"];
		let mut state = self.grid as usize;
		let mut i: u8 = 1;
		while i != 10 {
			let mut square = symbols[state % 3].to_owned();
			if square == "_" {
				square = i.to_string();
			}
			print!("{} ", square);
			state /= 3;
			if (i % 3) == 0 {
				println!();
			}
			i += 1;
		}
	}

	fn _solved(&self) -> bool {
		if self.grid == 3 {}
		true
	}

	fn choose_spot(&mut self, spot: u16, turn: &State) {
		if spot != 0 {
			self.grid += turn.as_uint16() + 3_u16.pow((9 - spot).into());
		} else {
			self.grid += turn.as_uint16();
		}
	}

	fn get_spot_owner(&self, spot: u16) -> State {
		let mut i = 0;
		let mut grid = self.grid;
		while i < spot {
			grid /= 3;
			i += 1;
		}
		match grid % 3 {
			0 => State::Blank,
			1 => State::FirstPlayer,
			2 => State::SecondPlayer,
			_ => State::Blank,
		}
	}
}

fn abort() {
	std::process::exit(0);
}

fn help() {
	println!("(H)elp  => Prints this help screen");
	println!("(Q)uit | Abort | Exit  => Exit the app");
}

fn computer() {
	play()
}

fn spot_taken(handler: &TicTacToe, spot: u16) -> bool {
	handler.get_spot_owner(spot) != State::Blank
}

fn friend() {
	let mut turn = State::FirstPlayer;
	let mut handler = TicTacToe::default();

	loop {
		println!(
			"Player {:?}'s turn!",
			if turn == State::FirstPlayer { 1 } else { 2 }
		);

		handler.print();

		let number: String = prompt("Choose a number!");
		let chosen_number: u16 = match number.parse() {
			Ok(n) => n,
			Err(_) => continue,
		};
		if !spot_taken(&handler, chosen_number) {
			handler.choose_spot(chosen_number, &turn);
		} else {
			println!(
				"There is already something occupying that spot! Please enter another number!"
			);
			continue;
		}
		turn = turn.next();
	}
}

fn online() {}

fn play() {}

fn main() {
	loop {
		println!("Welcome to Tic-Tac-Toe!");
		println!("You can play against a (c)omputer, your (f)riend, or someone (o)nline!");
		let mode: String = prompt("Which one do you prefer? (computer/friend/online)");
		match mode.trim().to_uppercase().as_str() {
			"ABORT" | "EXIT" | "Q" | "QUIT" => abort(),
			"HELP" | "H" => help(),
			"COMPUTER" | "C" => computer(),
			"FRIEND" | "F" => friend(),
			"ONLINE" | "O" => online(),
			_ => continue,
		}
		println!("Hi!");
	}
}
