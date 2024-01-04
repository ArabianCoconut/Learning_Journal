// Author: ArabianCoconut
// Date: 28/12/2023
use std::{io,process::exit};
use indoc::indoc;
mod library;
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
      5. Rectangle Area
      6. Employee Data
      7. Median and Mode
      8. Pig Latin\n
      Note: Type q to quit
    "};
    println!("{}", program_text);
		let example= vec![1,2,3,4,3];
    let example_2 = "Hello World";
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
		println!("You chose option: {}", input);
		let input = input.trim();
		match input {
		"1" => library::guessing_game(),
		"2" => library::temperature_converting(180.0, 70.0),
		"3" => library::fibonacci(10),
		"4" => library::twelve_days_of_christmas(),
		"5" => library::rectangle_area(10, 20),
    "6" => library::employee_data(),
    "7" => library::median_mode(example.clone(),example.len()),
    "8" => library::pig_latin(example_2),
		"q" => exit(0),
		_   => println!("Please type a number between 1 and 4 or q to quit")
		}

}