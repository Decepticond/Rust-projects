/*

Aleksandr S
March 2, 2024

*/


use std::io;

fn main() {
    loop {
        println!("Please enter a nth term to get its fibonacci number: ");
        let mut fibonacci_term = String::new();

        io::stdin()
            .read_line(&mut fibonacci_term)
            .expect("Failed to read line.");

        let fibonacci_term_num: u64 = fibonacci_term
            .trim()
            .parse()
            .expect("Failed to read line.");

        let result = fibonacci_sequence(fibonacci_term_num);
        println!("The {}th fibonacci number is {}", fibonacci_term_num, result)

    }
}

fn fibonacci_sequence(num: u64) -> u64 {
    if num <= 1 {
        return num;
    } else {
        return fibonacci_sequence(num-1) + fibonacci_sequence(num-2)
    }
}
