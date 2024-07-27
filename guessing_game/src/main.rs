use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..101);
    let mut input = String::new();
    let mut number: i32;

    println!("Guess the number!");

    loop {
        println!("Please input your guess:");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                number = num;
                if number == secret_number {
                    println!("Congratulations! You guessed the correct number!");
                    break;
                } else if number > secret_number {
                    println!("Too high!");
                } else {
                    println!("Too low!");
                }
            }
            Err(_) => {
                println!("Please type a valid number!");
            }
        }
    }
}
