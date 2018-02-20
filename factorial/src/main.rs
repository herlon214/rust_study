use std::io;

fn factorial(n: u64) -> u64 {
    if n == 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn main() {
    println!("@ Factorial Number @");
    println!("Type a number:");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Fail to read number");
    let number: u64 = number.trim().parse().expect("Fail to parse number");

    println!("The factorial of {} is {}", number, factorial(number));
}
