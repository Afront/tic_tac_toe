use promptly::prompt;

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
	fn print(self) {
		let symbols = ["_", "O", "X"];
		let mut state = 19683;
		let mut i: i32 = 0;
		while state != 1 {
			print!("{} ", symbols[state % 3]);
			state /= 3;
			i += 1;
			if (i % 3) == 0 {
				println!();
			}
		}
	}
}

fn abort() {
	std::process::exit(0);
}

fn help() {
	println!("(H)elp  => Prints this help screen");
	println!("(Q)uit | Abort | Exit  => Exit the app");
	println!("Sign (U)p | Register | Signup => Sign up to the app");
	println!("Sign (I)n | Signin | Login | Log in | Log on => Sign in to the app");
}

fn computer() {
	play()
}

fn friend() {}

fn online() {}

fn play() {
	let handler = TicTacToe::default();
	handler.print();
}

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
	}
}
