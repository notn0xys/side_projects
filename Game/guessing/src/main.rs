use std::io;
use rand::Rng;
fn main() {
    let mut input = String::new();
    let secret = rand::thread_rng().gen_range(0..=100);
    let mut higher = 100;
    let mut lower = 0;
    loop {
        let mut guess = 0;
        println!("Enter Your guess: ");
        io::stdin().read_line(&mut input).expect("Failed to read");
        match input.trim().parse() {
            Ok(1..=100) => {
            guess = input.trim().parse::<i32>().expect("failed to read");
            input.clear();
            },
            Ok(_) => {
                println!("Enter a number between 1-100");
                input.clear();
                continue;
            },
            Err(_) => {
                println!("Enter an integer");
                input.clear();
                continue;
            }
        }
        
            if guess > secret{
                higher = guess;
                println!("{} < ??? < {}" , lower,higher);
            }
            else if guess < secret{
                if lower > guess{
                    println!("{} < ??? < {}" , lower,higher);
                }
                else{
                    lower = guess;
                    println!("{} < ??? < {}" , lower,higher);
                }    
            }
            else{
            println!("You guessed correctly the secret number is {} ",secret);
            break;
            }
    }
    println!("Game end");
  
}
