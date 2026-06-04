use std::{thread, time::Duration};

use rand::Rng;

use crate::input;

fn transaction_animation() {
    println!("Initializing Transaction");

    for i in 3..=1 {
        thread::sleep(Duration::from_secs(1));
        println!(". . . {i} . . .");
        thread::sleep(Duration::from_secs(1));
    }

    println!("Transaction Completed");
}

fn deposit_money(balance: &mut f64) {
    loop {
        let amount_to_deposit: f64 = match input("Enter amount to deposit: ").parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Given input is not a number!");
                continue;
            }
        };

        if amount_to_deposit < 0.00 {
            println!("Invalid amount to deposit");
            continue;
        } else {
            *balance += amount_to_deposit;
            transaction_animation();
        }
    }
}

fn withdrawing_money(balance: &mut f64) {
    loop {
        let amount_to_withdraw: f64 = match input("Enter amount to withdraw: ").parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Given input is not a number!");
                continue;
            }
        };

        if amount_to_withdraw < 0.00 {
            println!("Invalid amount to withdraw");
            continue;
        } else if amount_to_withdraw > *balance {
            println!("Insufficient balance");
            continue;
        } else {
            *balance -= amount_to_withdraw;
            transaction_animation();
        }
    }
}

pub fn run() {
    println!("Welcome to the Banking Program");

    let mut balance: f64 = rand::thread_rng().gen_range(1.00..=999999.00);

    loop {
        println!("Current Balance: ₹{balance}");
        println!("Type 'deposit money' to deposit money");
        println!("Type 'withdraw money' to withdraw money");
        println!("Type 'exit' to return to main menu");

        let cmd: String = input("Enter a command: ");

        match cmd.as_str() {
            "deposit money" => deposit_money(&mut balance),
            "withdraw money" => withdrawing_money(&mut balance),
            "exit" => {
                println!("Return to main menu...");
                return;
            }
            _ => println!("Invalid command. Please try again"),
        }
    }
}
