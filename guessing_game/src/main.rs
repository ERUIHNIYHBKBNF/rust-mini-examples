use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess number");

    let answer = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input your guess number or 'exit'...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "exit" {
            println!("Bye~");
            break;
        }

        let guess: u32 = guess.trim().parse().expect("Invalid input");

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
