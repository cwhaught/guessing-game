use inquire::Confirm;

fn main() {
    let mut play_game = false;

    let ans = Confirm::new("Do you want to play a guessing game?")
        .with_default(false)
        .prompt();

    match ans {
        Ok(true) => {
            println!("Great, that's what I am going to figure out next.");

            play_game = true;
        },
        Ok(false) => println!("Well that's easy, I don't have a game yet anyway."),
        Err(_) => println!("ohhh noe"),
    }

    if(play_game){
        println!("You won!");
    } else {
        println!("Pretend you're on the main screen of a game.");
    }
}
