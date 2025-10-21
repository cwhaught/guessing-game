use inquire::{Confirm, CustomType};
use inquire::validator::Validation;
use rand::Rng;

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
        Ok(true) => return true,
        Ok(false) => println!("Lame. I didn't want to play with you anyway."),
        Err(_) => println!("ohhh noe"),
    }

    false
}

fn num_guessing_game() -> i8 {
    let random_num:i8 = loop {
        let n = rand::rng().random_range(1..101);
        if n != 67 {
            break n;
        }
    };

    let num_guessing_prompt = CustomType::<i8>::new ("I'm thinking of a number between 1 - 100, try to guess it.")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Enter a number between 1 and 100")
        .with_validator(move |input:&i8| {
            if *input == 67 {
                Ok(Validation::Invalid("Absolutely not, you know why".into()))
            } else if (*input > 0 && *input < 101) && *input != random_num {
                if *input > random_num {
                    Ok(Validation::Invalid("You guessed too high".into()))
                } else {
                    Ok(Validation::Invalid("You guessed too low".into()))
                }
            } else if *input == random_num {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Enter a value between 1 - 100".into()))
            }
        })
        .prompt();

    match num_guessing_prompt {
        Ok(guess) => {
            println!("YOU GOT IT! My number was {}", guess);
            guess
        },
        Err(_) => {
            println!("ohhh no");
            -1
        },
    }
}
