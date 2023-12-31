// Author: ArabianCoconut
// Date: 28/12/2023
use std::{io,process::exit};
use indoc::indoc;
mod guessing_game;
mod learning;
fn main() {
    //take user input
    let mut input = String::new();
    let program_text=indoc!{"
    Author: ArabianCoconut
    Description: Basic CLI program for practice from The Rust book.\n
      Please choose an option:
      1. Guessing Game
      2. Temperature Converting
      3. Fibonacci
      4. Twelve Days of Christmas
      5. Rectangle Area\n
      Note: Type q to quit
    "};
    println!("{}", program_text);
		io::stdin().read_line(&mut input).expect("Failed to read line");
		print!("You chose option: {}\n", input);
		let input = input.trim();
		match input {
		"1" => guessing_game::guessing_game(),
		"2" => learning::temperature_converting(180.0, 70.0),
		"3" => learning::fibonacci(10),
		"4" => learning::twelve_days_of_christmas(),
		"5" => learning::rectangle_area(10, 20),
		"q" => exit(0),
		_   => print!("Please type a number between 1 and 4 or q to quit\n")
		}
}