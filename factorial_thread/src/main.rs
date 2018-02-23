extern crate time;

use std::thread;
use std::cmp;
use std::io;
use time::PreciseTime;

fn factorial_range (starting: u64, ending: u64) -> u64 {
    let mut result = 1;

    for i in starting..ending {
        result = result * i;
    }

    return result;
}

fn main() {
    let mut children = vec![];
    let mut results: Vec<u64> = vec![];
    let mut number = String::new();
    let mut max_threads = String::new();

    println!("@ Thread Factorial Calculation @");

    println!("Type the max threads used to calc:");
    io::stdin().read_line(&mut max_threads).expect("Fail to read the number");
    let max_threads: u64 = match max_threads.trim().parse() {
        Ok(num) => num,
        Err(_) => 1
    };

    println!("Type a number to calc: ");
    io::stdin().read_line(&mut number).expect("Fail to read the number");
    let number: u64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 1
    };

    // Start the timer
    let start = PreciseTime::now();

    // Calculate the chunk size
    let chunks: u64 = number / max_threads;
    let mut actual_number = 1;

    if max_threads > 1 {
        println!("Using {} threads...", max_threads);
        while actual_number < number + 1 {
            children.push(thread::spawn(move || -> u64 {
                let result = factorial_range(actual_number, cmp::min(actual_number + chunks, number));
                result
            }));
            actual_number = actual_number + chunks;
        }

        for child in children {
            results.push(child.join().unwrap());
        }
    } else {
        println!("Single thread...");
        results.push(factorial_range(1, number));
    }

    // Reduce the values
    let mut response: u64 = 1;
    for result in results {
        response *= result;
    }

    // We need to multiply by the number because factorial_range doesn't multiply the ending number
    response *= number;

    let end = PreciseTime::now();

    println!("Response: {:#?}", response);
    println!("{} seconds elapsed", start.to(end));
}
