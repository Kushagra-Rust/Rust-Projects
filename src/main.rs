mod calculator;
mod banking_program;
mod number_guessing_game;
mod rock_paper_scissors;

fn input(prompt: &str) -> String {
    use std::io::{self, Write};

    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => return input.trim().to_lowercase(),
            Err(err) => println!("Error Reading Input: {err}"),
        }
    }
}

fn show_menu() {
	println!("Type 'help' to see the available options.");
    println!("Type 'files' to see executable files");
    println!("Type a 'file's name' to run that file");
    println!("Type 'exit' to quit the application");
}

fn files() {
    println!("1. Calculator");
    println!("2. Banking Program");
    println!("3. Number Guessing Game");
    println!("4. Rock Paper Scissors");
    println!("More Coming Soon...");
}

fn main() {
    println!("Welcome to main Rust cli application");
    show_menu();

    loop {
        let user_cmd: String = input(">> ");

        match user_cmd.as_str() {
            "help" => show_menu(),
            "files" => files(),
            "calculator" | "1" => calculator::run(),
            "banking program" | "2" => banking_program::run(),
            "number guessing game" | "3" => number_guessing_game::run(),
            "rock paper scissors" | "4" => rock_paper_scissors::run(),
            "exit" => {
                println!("Exiting application");
                break;
            }
            _ => println!("Invalid command. Type 'help' for available commands")
        }
    }
}
