use std::io;

fn main() {

    let mut input = String::new();
    let base: i32;
    let exponent: i32;
    let result: i32;

    println!("Enter the base: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    base = input.trim().parse().expect("Please type a number!");
    println!("Enter the exponent: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    exponent = input.trim().parse().expect("Please type a number!");

    result = calculate_exponent(base, exponent);
    println!("The result of {} ^ {} is: {}", base,exponent,result);
    
}

fn calculate_exponent(base: i32, exponent: i32) -> i32 {
    if exponent == 0{
        return 1;
    } else {
        return base * calculate_exponent(base, exponent - 1);
 
    }
}