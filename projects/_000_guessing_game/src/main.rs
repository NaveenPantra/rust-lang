use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("\n\nGuess the number\n");
	let secrete_number = rand::thread_rng().gen_range(1..=100);
	loop {
		println!("\n\nPlease input your guess: ");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Failed to read line");
		guess = guess.trim().to_string();
		println!("You guess: {guess}");
		let guess: u32 = match guess.parse() {
			Ok(num) => num,
			Err(_) => {
				if guess.to_lowercase() == "quit".to_string() {
					println!("Quitting game!");
					std::process::exit(0);
				}
				println!("Please enter number with in limits 0 to 100 both included");
				continue;
			},
		};
		match guess.cmp(&secrete_number) {
			Ordering::Less => println!("\nToo small!\n"),
			Ordering::Greater => println!("\nToo big!\n"),
			Ordering::Equal => {
				println!("\n\n\nYou won!\n\n");
				break;
			}
		};
	}
}
