use inquire::{Confirm, CustomType};

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

    return false;
}

fn num_guessing_game() -> i8 {
    let num_guessing_prompt = CustomType::<i8>::new ("Guess a number 1 - 100")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("That is not an acceptable input, try again")
        .prompt();

    match num_guessing_prompt {
        Ok(guess) => {
            println!("Your number is {}", guess);
            return guess;
        },
        Err(_) => {
            println!("ohhh no");
            return -1;
        },
    }
}
