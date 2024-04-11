use std::io;
use rand::Rng;

fn say_rules() {
    println!("Number Guessing Game");
    println!("Guess a number between 1-100");
}

fn get_user_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            input.truncate(input.len() - 1);
        }
        Err(error) => println!("error: {error}"),
    }

    return input;
}

fn get_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..100);
    return n;
}


fn main() {
    say_rules();
    let mut guesses = 3;
    let mut done: bool = false;
    let ran_num = get_random_number();
    while !done {
        let input = get_user_input().parse::<u32>().unwrap();
        if input == ran_num {
            println!("Congrats you got it !!");
        } else {
            println!("Oh No! You didn't quite get it.");
        }

        guesses = guesses + 1;
        if guesses % 3 == 0 {
            done = true;

            println!("Sorry you ran out of guesses!");
            println!("The number was {}", ran_num)
        }
    }
}