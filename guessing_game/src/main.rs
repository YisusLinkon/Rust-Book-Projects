use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Now welcome to the guessing game!");
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret}");

    loop {

        println!("Please enter a number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");
        
        // This will trigger an error
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");


        // This other option allows me how to handle the differents results, Ok and Err.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congrats!!!");
                break;
            }
        }

    }

}
