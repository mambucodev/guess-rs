use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    clear();

    let maximum = 20;

    let index = rand::thread_rng().gen_range(1..=maximum);

    println!(
        "{}",
        format!(
            "Guess a number from {} to {}!",
            "1".yellow().bold(),
            maximum.to_string().yellow().bold()
        )
        .green()
    );

    'outer: loop {
        let mut guess = String::new();

        print!("{} ", ">".cyan().bold());
        io::stdout().flush().expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        if guess.trim().to_lowercase() == "exit" {
            println!(
                "{}",
                "You're weren't even made for guessing games after all..."
                    .red()
                    .bold()
            );
            break 'outer;
        }

        let parsed_guess = guess.trim().parse::<i32>();
        match parsed_guess {
            Ok(n) => {
                if n > maximum {
                    println!(
                        "{}\n{}",
                        format!("Why are you guessing a number higher than {maximum}...").red(),
                        "Who do you think you are?".red().bold()
                    );
                    continue;
                } else if n < 1 {
                    println!("{}", "Don't worry, there's no negative numbers.".red());
                    continue;
                }
                match n.cmp(&index) {
                    Ordering::Less => println!("{}", "Go higher...".red().bold()),
                    Ordering::Greater => println!("{}", "Go lower...".red().bold()),
                    Ordering::Equal => {
                        println!(
                            "{} It was {}.",
                            "You won!".green().bold(),
                            n.to_string().yellow().bold()
                        );
                        break 'outer;
                    }
                }
            }
            Err(_) => {
                println!("{}", format!("Seems like that wasn't a number after all...\nUnless it was higher than {}.", "2,147,483,647".yellow().bold()).red());
            }
        };
    }
}
