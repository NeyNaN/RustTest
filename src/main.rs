use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn greetings() {
    let text_var = "variable";
    println!("Hello, world!");
    println!(
        "Testing formatting of {} {} {}",
        "formatted", text_var, "inputs"
    )
}

fn guessing() {
    
    let target = rand::thread_rng().gen_range(1..=100);
    
    // println!("Target is {target}");

    println!("Guess the number!");
    println!();

    let mut i: u32 = 0;
    loop {
        i += 1;
        let mut guess = String::new();

        println!("Enter guess: ");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess");
                continue;
            }
        };
        
        println!("You guessed {guess}");
    
        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct! {i} guesses used!");
                break;
            }
        }
    }
}

fn main() {
    greetings();
    guessing();
}
