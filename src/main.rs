use rand::Rng;
use std::{cmp::Ordering, io, process::exit};

fn main() {
    let random_num = rand::thread_rng().gen_range(0..=100);

    println!("try to guess the number!");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("it went to shit");

        let guess = guess.trim();
        let guess: i32 = match guess {
            "quit" => exit(0),
            _ => match guess.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("not a number, try again");
                    continue;
                }
            },
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("guessed too small"),
            Ordering::Greater => println!("guessed too big"),
            Ordering::Equal => {
                println!("you found it!");
                break;
            }
        }
    }
}
