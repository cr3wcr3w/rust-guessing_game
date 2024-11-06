// std mean standard library
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    // infinite loop
    loop {
        println!("Please input your guess.");

        // variable in default is immutable
        // mut = becomes the variable into mutable
        let mut guess = String::new();

        // & = reference, reference in default is immutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing
        // trim = remove all spaces
        // parse = convert the type to other type (in this case the variable is u32)
        // expect = error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // handling non u32 type
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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

// cargo update
// cargo doc --open
