
use std::io;

fn fibo(n: u64) -> u64 {
  if n == 0 {
    return 0
  } else if n ==  1 {
    return 1
  } else {
    return fibo(n -1) + fibo(n - 2)
  }

}

fn main() {
  println!("@ Calculating Fibonacci Number @");
  println!("Type a number:");

  let mut number = String::new();
  io::stdin().read_line(&mut number).expect("Fail to read line");
  let number: u64 = number.trim().parse().expect("Fail to read number");

  println!("The {}nth fibo number is {}", number, fibo(number))
}
