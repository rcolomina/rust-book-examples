
use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {

    println!("Guess the number!");


    let secret_number = rand::thread_rng().gen_range(1..=100); // see chapter 10 for more like this

    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your guess.");


        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)   // this put iinpu into guess but also returns Result value (see chapter 6)
            .expect("Failed to read line");   // see recovering from error in chapter 9

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input parsing error");
                continue;}
        }; //.expect("Plase type a number!"); // shadowing guess variable, see chapter 3

        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {   // see chapter 6 and chapter 18
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!");
                                break;
            }
        }
    }    
    
}
