use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input your giss, thanks.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse()
            .expect("Please type number bro");

        println!("YOu guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}


//https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html