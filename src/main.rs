use inquire::{Confirm, CustomType};
use inquire::validator::Validation;

fn main() {
    if play_game() {
        num_guessing_game();
    } else {
        println!("Pretend you're on the main screen of a game.");
    }
}

fn play_game() -> bool {
    let play_game_prompt = Confirm::new("Do you want to play a guessing game?")
        .with_default(false)
        .prompt();

    match play_game_prompt {
        Ok(true) => {
            println!("Great, that's what I am going to figure out next.");

            return true;
        },
        Ok(false) => println!("Well that's easy, I don't have a game yet anyway."),
        Err(_) => println!("ohhh noe"),
    }

    false
}

fn num_guessing_game() -> i8 {
    let num_guessing_prompt = CustomType::<i8>::new ("I'm thinking of a number between 1 - 100, try to guess it.")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Enter a number between 1 and 100")
        .with_validator(|input:&i8| {
            if *input == 67 {
                Ok(Validation::Invalid("Absolutely not, you know why".into()))
            }else if *input > 0 && *input <= 100 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("You must enter a value between 1 - 100".into()))
            }
        })
        .prompt();

    match num_guessing_prompt {
        Ok(guess) => {
            println!("Your number is {}", guess);
            guess
        },
        Err(_) => {
            println!("ohhh no");
            -1
        },
    }
}
