use inquire::{CustomType, InquireError, Select};
use inquire::validator::Validation;
use rand::Rng;

fn main() {
    const GAME_NUM_GUESS: &str = "Number Guessing";
    const GAME_MATCHING: &str = "Matching";
    const GAME_DONE: &str = "I'm Done";

    let game_options: Vec<&str> = vec![GAME_NUM_GUESS, GAME_MATCHING, GAME_DONE];

    let choice = play_game(game_options);

    if choice == GAME_NUM_GUESS {
        num_guessing_game();
    } else if choice == GAME_MATCHING {
        num_guessing_game();
    } else {
        println!("See you next time!");
    }
}

fn play_game(game_options: Vec<&str>) -> &str {
    let ans: Result<&str, InquireError> = Select::new("What kind of game do you want to play?", game_options).prompt();

    match ans {
        Ok(choice) => {
            println!("{}! Great! Let's do it!", choice);
            choice
        },
        Err(_) => {
            println!("Beep boop, something went wrong...");
            ""
        },
    }
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
                    Ok(Validation::Invalid(format!("{input} is too high").into()))
                } else {
                    Ok(Validation::Invalid(format!("{input} is too low").into()))
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
