use inquire::Confirm;

fn main() {
    if(play_game()){
        println!("You won!");
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
