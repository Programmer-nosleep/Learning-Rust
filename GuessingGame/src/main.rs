use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("welcome to the guessing game!");
    println!("I\'m thinking of a number between 1 and 100. can you guess it?");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("please input your guess : ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number.");
                continue;
            }
        };

        println!("you guessed : {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small! try again."),
            Ordering::Greater => println!("too big! try again."),
            Ordering::Equal => {
                println!("congratulations! you guessed the number");
                break;
            }
        }
    }
}
