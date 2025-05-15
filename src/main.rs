use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

// Clear the console screen
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    clear();

    let mut maximum: usize = 20;
    let mut lives: usize = 5;
    let mut level: usize = 1;

    // Display game instructions
    println!("{}", "Welcome to the Number Guessing Game!".green().bold());
    println!("Instructions:");
    println!("1. Guess a number within the given range.");
    println!("2. You have {lives} lives to start with.");
    println!("3. Each correct guess advances you to the next level.");
    println!("4. The range increases by 10 for each level.");
    println!("5. Type 'exit' at any time to quit the game.");
    println!();

    loop {
        println!(
            "{}",
            format!(
                "Level {}: Guess a number from {} to {}!",
                level,
                "1".yellow().bold(),
                maximum.to_string().yellow().bold()
            )
            .green()
        );
        println!(
            "{}",
            format!("Lives remaining: {}", "❤".repeat(lives)).red()
        );

        let index = rand::thread_rng().gen_range(1..=maximum);

        'inner: loop {
            let mut guess = String::new();

            print!("{} ", ">".cyan().bold());
            io::stdout().flush().expect("Failed to flush stdout.");

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");

            if guess.trim().to_lowercase() == "exit" {
                println!("{}", "Game over. Thanks for playing!".red().bold());
                return;
            }

            let parsed_guess = guess.trim().parse::<u32>();
            match parsed_guess {
                Ok(n) => {
                    if (n as usize) > maximum {
                        println!(
                            "{}",
                            format!("Please guess a number between 1 and {maximum}.").red()
                        );
                        continue;
                    }
                    match (n as usize).cmp(&index) {
                        Ordering::Less => {
                            println!("{}", "Too low! Try a higher number.".red().bold());
                            lives -= 1;
                        }
                        Ordering::Greater => {
                            println!("{}", "Too high! Try a lower number.".red().bold());
                            lives -= 1;
                        }
                        Ordering::Equal => {
                            println!(
                                "{} It was {}.",
                                "Correct!".green().bold(),
                                n.to_string().yellow().bold()
                            );
                            level += 1;
                            maximum += 10;
                            lives += (level as f32 / 2.0).round() as usize;
                            println!(
                                "{}",
                                format!(
                                    "You've advanced to level {} and gained an extra life!",
                                    level
                                )
                                .yellow()
                                .italic()
                            );
                            break 'inner;
                        }
                    }
                }
                Err(e) => match e.kind() {
                    std::num::IntErrorKind::PosOverflow => {
                        println!(
                            "{}",
                            format!(
                                "Number too large. Please enter a number below {}.",
                                u32::MAX
                            )
                            .red()
                        );
                    }
                    _ => {
                        println!(
                            "{}",
                            "Invalid input. Please enter a valid number.".red().bold()
                        );
                    }
                },
            };

            if lives == 0 {
                println!(
                    "{}",
                    format!("Game Over, it was {index}! You made it to level {}.", level)
                        .red()
                        .bold()
                );
                return;
            }

            println!(
                "{}",
                format!("Lives remaining: {}", "❤".repeat(lives)).red()
            );
        }

        clear();
    }
}
