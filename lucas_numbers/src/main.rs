use std::io;

fn lucas(n: u64) -> u64 {
    if n == 0 {
        return 2;
    } else if n == 1 {
        return 1;
    } else {
        return lucas(n - 1) + lucas(n - 2);
    }
}

fn main() {
    println!("@ Lucas Numbers @");

    println!("Type the nth number:");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Fail to read the number");
    let number: u64 = number.trim().parse().expect("Fail to parse the number");

    println!("Result: {}", lucas(number));
}
