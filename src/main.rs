use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let _secret_num = rand::thread_rng().gen_range(1..3);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess); 

        match guess.cmp(&_secret_num) {
            Ordering::Less =>{ 
                println!("Too small!");
                break;
                }
            Ordering::Greater => { 
                println!("Too big!");
                break;
                }
            Ordering::Equal => { 
                println!("You Guess the Correct Number, You Win!");
                break;
            }
        }
    } 

}