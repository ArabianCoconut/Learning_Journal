//Author: ArabianCoconut
//Date: 30/12/2023
// Description: Basic code for practice from The Rust book.
// Ref: https://doc.rust-lang.org/book/ch03-05-control-flow.html

pub fn temperature_converting(c: f32, f: f32){
    // Comment: A simple temperature converting function
    let formula_celsius :f32= (f-32.0)*(0.55);
    let formula_fahrenheit :f32= (c*1.8)+(32.0);
    return println!("{}°C = {}°F\n{}°F = {}°C", c,formula_fahrenheit, f,formula_celsius);
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
    for i in 0..12{
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