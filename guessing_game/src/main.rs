extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1, 101);
  loop {
    println!("***** Guess the number! *****");
    println!("Please input your guess:");

    let mut guess = String::new();

    // read_line() returns a Result instance. If its value is Ok, expect() will return the value
    // If its value is Err, `expect` will cause the program to crash an display the message passed
    // as an argument.
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    // Shadow the previous guess variable and converts it to the number type u32 with parse()
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    // & indicates that this argument is a reference
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
