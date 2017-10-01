extern crate rand;
use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    main_guess_number()
}

fn main_guess_number() {
    println!("Guess number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        print!("Please insert your number ({}): ", secret_number);
        io::stdout().flush().expect("Failed to flush standard output");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(x) => {
                println!("Dino {}", x);
                continue
            }
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            },
        }
    }
}
