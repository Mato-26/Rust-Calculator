use std::io;

fn main() {

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();
    let mut result = String::new();

    println!("Welcome to this calculator!");

    /*

    io::stdin().read_line(&mut num1).unwrap();

    is a simpler way of doing this:

    num1 = io::stdin().read_line(&mut num1).unwrap();

    NOTE: it looks a lot cleaner tho :p

     */

    println!("Please insert the first number: ");
    io::stdin().read_line(&mut num1).unwrap();

    println!("Please insert the second number: ");
    io::stdin().read_line(&mut num2).unwrap();

    println!("What is the calculation method? ");
    io::stdin().read_line(&mut operator).unwrap();

    let num1: i32 = num1.trim().parse().unwrap();
    let num2: i32 = num2.trim().parse().unwrap();

    match operator.trim() {
        "+" => {
            result = (num1 + num2).to_string();
            println!("You've chosen to add the numbers together")
        },
        "-" => {
            result = (num1 - num2).to_string();
            println!("You've chosen to subtract the numbers")
        },
        "*" => {
            result = (num1 * num2).to_string();
            println!("You've chosen to multiply the numbers")
        },
        "/" => {
            result = (num1 / num2).to_string();
            println!("You've chosen to divide the numbers")
        },
        _ => println!("You've chosen an invalid operator")
    };

    print!("The result is: {}", result)

}
