#[allow(unused_must_use)]

use std::io;

fn main() {
    let mut text = String::new();

    println!("Please, enter a text:");
    // read data from a console
    io::stdin()
        .read_line(&mut text)
        .expect("Faild to read a line");

    println!("Entered text: {text}");

    text = String::new();
    println!("Please, enter a number:");
    // read data from a console
    io::stdin()
        .read_line(&mut text)
        .expect("Faild to read a line");

    println!("You entered: {}", text);    
    let number: u32 = text
        .trim()
        .parse()
        .expect("Conversion is failed. Please enter a number");
    println!("Entered number: {number}");
}
