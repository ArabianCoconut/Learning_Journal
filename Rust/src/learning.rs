//Author: ArabianCoconut
//Date: 30/12/2023
// Description: Basic code for practice from The Rust book.

pub fn temperature_converting(c: f32, f: f32){
    // Comment: A simple temperature converting function
    let formula_celsius :f32= (f-32.0)*(0.55);
    let formula_fahrenheit :f32= (c*1.8)+(32.0);
    return println!("{}°C = {}°F\n{}°F = {}°C", c,formula_fahrenheit, f,formula_celsius);
    
}

//TODO: Generate the nth Fibonacci number

// TODO: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.