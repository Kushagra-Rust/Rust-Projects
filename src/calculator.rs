use crate::input;

fn get_number(prompt: &str) -> f64 {
    loop {
        let num: f64 = match input(prompt).parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Given input is not a number");
                continue;
            }
        };

        return num;
    }
}

fn calculate() {
    loop {
        let num1: f64 = get_number("Enter the first number: ");
        let operator: String = input("Enter the operator(+ -, *, /): ");
        let num2: f64 = get_number("Enter the second number: ");
        
        let result: f64 = match operator.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Error: Cannot divide by zero!");
                    continue;
                }
            }
            _ => {
                println!("Invalid operator. Please enter +, -, * or /!");
                continue;
            }
        };

        println!("Result: {result}");
        break;
    }
}

pub fn run() {
    println!("Welcome to the Calculator!");

    loop {
        println!("Type 'calculate' to perform a calculation.");
        println!("Type 'exit' to return to main menu.");

        let cmd: String = input("Enter a command: ");

        match cmd.as_str() {
            "calculate" => calculate(),
            "exit" => {
                println!("Returning to main menu...");
                return;
            }
            _ => println!("Invalid command. Please try again."),
        }
    }
}
