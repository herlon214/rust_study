extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1, 51);
  println!("Guest the number!");

  println!("Input your guess: ");

  loop {
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too low!"),
      Ordering::Greater => println!("Too high!"),
      Ordering::Equal => { 
        println!("You guessed!");
        break;
      }
    }
  }
  
}