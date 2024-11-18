use std::io;
use std::time::Instant;

// The Collatz function: If n is even, divide by 2. If n is odd, multiply by 3 and add 1.
fn collatz(number: i64) -> i64 {
    if number % 2 == 0 {
        number / 2
    } else {
        3 * number + 1
    }
}

fn timed_collatz_sequence() {
    loop {
        println!("Enter number: ");
        let mut input = String::new();

        // Read user input from stdin
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse the input and handle potential errors
        match input.trim().parse() {
            Ok(mut number) => {
                // Start timing the sequence
                let start_time = Instant::now();

                // Continue applying the Collatz function until we reach 1
                while number != 1 {
                    number = collatz(number);
                    println!("{}", number);
                }

                // Calculate and print the elapsed time
                let elapsed_time = start_time.elapsed();
                println!("Time taken: {:.6} seconds", elapsed_time.as_secs_f64());
                break;
            }
            Err(_) => {
                // Handle invalid input
                println!("Error: You must enter an integer.");
                println!("Please try again.");
            }
        }
    }
}

fn main() {
    // Call the function to start the Collatz sequence calculation
    timed_collatz_sequence();
}
