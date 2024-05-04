#![deny(clippy::all, clippy::pedantic, clippy::style)]

use std::io;

fn main() {
    println!("Generate the Fibonacci sequence");

    let amount_of_numbers = get_input();


    println!("The Fibonacci sequence is:");

    for i in 0..amount_of_numbers {
        println!("{}", fibonacci(i));
    }
}

fn get_input() -> u32 {
    loop {
        println!("Please input a number.");

        let mut amount = String::new();

        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");

        println!("You entered: {amount}");

        let Ok(input) = amount.trim().parse() else {
            println!("Please enter a valid positive integer!");
            continue;
        };
        return input;
    }
}

fn fibonacci(n: u32) -> u32 {
    // For n = 0, the first two numbers in the Fibonacci sequence are 0 and 1.
    // For n > 1, the nth number in the sequence is the sum of the (n-1)th and (n-2)th numbers.
    // Aka F(n) = F(n-1) + F(n-2)
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
