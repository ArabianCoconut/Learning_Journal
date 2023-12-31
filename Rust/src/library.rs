//Author: ArabianCoconut
//Date: 30/12/2023
//Description: Basic code for practice from The Rust book.
//Ref: https://doc.rust-lang.org/book/ch03-05-control-flow.html

// Comment: A simple guessing game from the rust book
pub fn guessing_game(){
	use rand::Rng;
	use std::cmp::Ordering;
	use std::io;
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {guess}");

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}

pub fn temperature_converting(c: f32, f: f32){
		// Comment: A simple temperature converting function
		let formula_celsius :f32= (f-32.0)*(0.55);
		let formula_fahrenheit :f32= (c*1.8)+(32.0);
		println!("{}°C = {}°F\n{}°F = {}°C", c,formula_fahrenheit, f,formula_celsius)
}

//Generate the nth Fibonacci number
pub fn fibonacci(n: u32){
		let mut a = 0;
		let mut b = 1;
		let mut c = 0;
		for _i in 0..n{
				c = a + b;
				a = b;
				b = c;
		}
		println!("The {}th Fibonacci number is {}", n, c);
}

//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
pub fn twelve_days_of_christmas(){
		let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth",
								"ninth", "tenth", "eleventh", "twelfth"];
		let gifts: [&str; 12] = ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds",
								"Five gold rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking",
								"Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
		for (i, _) in days.iter().enumerate(){
			println!("On the {} day of Christmas my true love sent to me", days[i]);
			for j in (0..i+1).rev(){
				if j == 0 && i != 0{
					print!("And ");
				}
				println!("{}", gifts[j]);
			}
			println!();
		}
}
//Rectangle area
// ref: https://doc.rust-lang.org/book/ch05-03-method-syntax.html
pub fn rectangle_area(height: u32, width: u32){
	struct Rectangle{
			width: u32,
			height: u32,
	}
		//Comment: This is a method
	impl Rectangle{
				// Method Rectangle formula
		fn area(&self) -> u32{
			self.width * self.height
		}
	}
	let rect1 = Rectangle{
			width: height,
			height: width,
	};
	println!("The area of the rectangle is {} square pixels.", rect1.area());
}

	#[cfg(test)]
	mod tests{
	use super::*;
	#[test]
	fn test_temperature_converter(){
		assert_eq!(temperature_converting(180.0, 70.0), (), "temperature_converting() should return value");
	}
	#[test]
	fn test_fibonacci(){
		assert_eq!(fibonacci(10), (), "fibonacci() should return value");}
	#[test]
	fn test_twelve_days_of_christmas(){
		assert_eq!(twelve_days_of_christmas(), (), "twelve_days_of_christmas() should return value");}
	#[test]
	fn test_rectangle_area(){
		assert_eq!(rectangle_area(10, 20), (), "rectangle_area() should return value");}
	}