use std::io;
// use rand::Rng;

fn main() {
    let mut value = String::new();
    println!("Enter a number:- ");
    io::stdin().read_line(&mut value).expect("Failed to read line");
    println!("You Entered {}", value);
}