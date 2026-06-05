use crate::input;

fn get_user_choice() -> String {
    loop {
        let user_choice: String = input("Choose Rock, Paper or Scissors: ");

        match user_choice.as_str() {
            "rock" | "paper" | "scissors" => return user_choice,
            _ => println!("Invalid choice. Choose either rock, paper or scissors"),
        }
    }
}

fn get_computer_choice() -> String {
    use rand::Rng;

    let valid_choices: [&str; 3] = ["rock", "paper", "scissors"];

    let computer_choice: &str  = valid_choices[rand::thread_rng().gen_range(0..=2)];

    return computer_choice.to_string();
}

fn rock_paper_scissors(player_score: &mut u32, computer_score: &mut u32) {
    let user_choice: String = get_user_choice();
    let computer_choice: String = get_computer_choice();

    println!("Choices -> Player: {user_choice} | Computer: {computer_choice}");

    if user_choice == computer_choice {
        println!("It's a tie")
    } else if user_choice == "rock" && computer_choice == "scissors" || user_choice == "paper" && computer_choice == "rock" || user_choice == "scissors" && computer_choice == "paper" {
        println!("You won.");
        *player_score += 1;
    } else {
        println!("You lost.");
        *computer_score += 1;
    }
}


pub fn run() {
    println!("Welcome to the Rock Paper Scissors");
    let mut round: u32 = 1;
    let mut player_score: u32 = 0;
    let mut computer_score: u32 = 0;

    loop {
        println!("Type 'play' to start the game");
        println!("Type 'exit' to return to the main menu");

        let cmd: String = input("Enter a command: ");

        match cmd.as_str() {
            "play" => {
                println!("Round: {round}");
                rock_paper_scissors(&mut player_score, &mut computer_score);
                round += 1;
            }
            "exit" => {
                println!("Returning to the main menu...");
                return;
            }
            _ => println!("Invalid command.")
        }
    }
}
