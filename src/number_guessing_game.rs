use rand::Rng;

use crate::input;

fn get_difficulty() -> String {
    loop {
        println!("----- Difficulty Levels -----");
        println!("1. Easy - (1-10), 3 Tries");
        println!("2. Medium - (1-50), 7 Tries");
        println!("3. Hard - (1-100), 12 Tries");
        println!("4. Random - Chooses a random difficulty level");
        println!("-----------------------------");

        let mut difficulty: String = input("Enter difficulty: ");

        if difficulty == "random" || difficulty == "4" {
            let valid_difficulty: [&str; 3] = ["easy", "medium", "hard"];
            difficulty = valid_difficulty[rand::thread_rng().gen_range(0..=2)].to_string();
            println!("Randomly selected difficulty: {}", difficulty);
        }

        match difficulty.as_str() {
            "easy" | "1" => return "easy".to_string(),
            "medium" | "2" => return "medium".to_string(),
            "hard" | "3" => return "hard".to_string(),
            _ => println!("Invalid difficulty. Please try again."),
        }
    }
}

fn main_game_loop(random_number: u32, max_attempts: u32, max_number: u32, computer_score: &mut u32, player_score: &mut u32) {
    let mut attempts: u32 = 0;
    let mut has_won: bool = false;

    while max_attempts > 0 && !has_won {
        let guess: String = input(&format!("Enter your guess (1-{max_number}): "));
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        attempts += 1;

        if guess > max_number || guess < 1 {
            println!("Your guess is out of bounds. Please try again.");
            println!("Tries left: {}", max_attempts - attempts);
        } else if guess < random_number {
            println!("Too low! Try again.");
            println!("Tries left: {}", max_attempts - attempts);
        } else if guess > random_number {
            println!("Too high! Try again.");
            println!("Tries left: {}", max_attempts - attempts);
        } else {
            println!("Congratulations! You've guessed the number in {attempts} attempts!",);
            *player_score += 1;
            has_won = true;
        }
    }

    if !has_won {
        println!("Game Over! The correct number was {random_number}.");
        *computer_score += 1;
    }

    println!("Current Score - Player: {player_score}, Computer: {computer_score}");
}

fn again() -> bool {
    loop {
        let play_again: String = input("Do you want to play again? (yes/no): ");
        match play_again.as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}

pub fn run() {
    println!("Welcome to the Number Guessing Game!");

    let mut round: i32 = 1;
    let mut computer_score: u32 = 0;
    let mut player_score: u32 = 0;
    let mut play: bool = true;

    while play {
        let difficulty: String = get_difficulty();

        let (max_number, max_attempts) = match difficulty.as_str() {
            "easy" => (10, 3),
            "medium" => (50, 7),
            "hard" => (100, 12),
            _ => unreachable!(),
        };  

        let random_number: u32 = rand::thread_rng().gen_range(1..=max_number);

        println!("Round: {round}");
        main_game_loop(random_number, max_attempts, max_number, &mut computer_score, &mut player_score);
        round += 1;
      
        play = again()
    }

    println!("Final Score - Player: {player_score}, Computer: {computer_score}. ");
    println!("Returning to main menu...");
}
