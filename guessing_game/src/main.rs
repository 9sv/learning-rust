use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Rust practice https://github.com/9sv | PROJECT 1 - Guessing Game\n");
    println!("Guess the number !");

    let rng_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("What is your guess? ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("ion kno how to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a number dipshit");
                continue;
            },
        };

        match guess.cmp(&rng_num) {
            Ordering::Less => println!("You were too low !"),
            Ordering::Greater => println!("You were too high !"),
            Ordering::Equal => {
                println!("The number was {}, you got it !", rng_num);
                break;
            }
        }
    }
}
