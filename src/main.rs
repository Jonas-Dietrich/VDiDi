mod ollama;
mod random;
mod util;

use ollama::generate_mail;
use tokio::runtime::Runtime;
use std::io::{self, Write};

fn main() {
    // Creating the runtime for the async function for ollama.rs
    let runtime = Runtime::new().unwrap();

    loop {
        println!("Please choose an option:");
        println!("1. Generate email using Ollama");
        println!("2. Generate email using .....");
        println!("q. Quit");

        print!("Your choice: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        print!("\n");

        match choice {
            "1" => {
                // Call the generate_mail function from ollama.rs
                match runtime.block_on(generate_mail()) {
                    Ok(response) => println!("Generated mail:\n\n{}", response),
                    Err(e) => eprintln!("Error generating mail: {}", e),
                }
            }
            "2" => {
                println!("Generated mail:\n\n");
            }
            "q" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please choose a valid option.");
            }
        }
        print!("\n");
    }
}
