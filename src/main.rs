use inquire::{CustomType, InquireError, Select};
use inquire::validator::Validation;
use rand::Rng;

fn main() {
    const GAME_NUM_GUESS: &str = "Number Guessing";
    const GAME_MULTI: &str = "Multiplication Tables";
    const GAME_DONE: &str = "I'm Done";

    loop {
        let game_options: Vec<&str> = vec![GAME_NUM_GUESS, GAME_MULTI, GAME_DONE];
        let choice = play_game(game_options);

        if choice == GAME_DONE {
            break;
        } else {
            if choice == GAME_NUM_GUESS {
                num_guessing_game();
            } else if choice == GAME_MULTI {
                game_multiplication_tables();
            }
        }
    }

    println!("See you next time!");
}

fn play_game(game_options: Vec<&str>) -> &str {
    let ans: Result<&str, InquireError> = Select::new("What kind of game do you want to play?", game_options).prompt();

    ans.unwrap_or_else(|_| {
        println!("Beep boop, something went wrong...");
        ""
    })
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

fn game_multiplication_tables() -> i16 {
    let first_num:i8 = rand::rng().random_range(0..13);
    let second_num:i8 = rand::rng().random_range(0..13);
    let answer:i16 = (first_num as i16) * (second_num as i16);
    let question = format!("What's {first_num} times {second_num}?");

    let num_guessing_prompt = CustomType::<i16>::new (&question)
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Enter a valid number")
        .with_validator(move |input:&i16| {
            if *input == answer {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(format!("{input} isn't correct, try again").into()))
            }
        })
        .prompt();

    match num_guessing_prompt {
        Ok(guess) => {
            println!("YOU GOT IT! The Answer is {}", guess);
            guess
        },
        Err(_) => {
            println!("ohhh no");
            -1
        },
    }
}
