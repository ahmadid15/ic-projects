use std::io::{self, Write};

fn main() {
    println!("Welcome to iCODE Ahmadu Bello University Zaria Bootcamp May 2025!");
    println!("{}", "=".repeat(65));
    println!();

    // Ask for user's name
    print!("Please enter your name: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    let mut name = String::new();

    // Read user input
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            let name = name.trim(); // Remove trailing newline

            println!();
            if !name.is_empty() {
                println!(" Hello, {}! ", name);
                println!();
                println!("Welcome to the iCODE Ahmadu Bello University Zaria Bootcamp!");
                println!("May 2025 Cohort");
                println!();
                println!("Get ready for an amazing coding journey!");
                println!("You're about to learn incredible programming skills");
                println!("Connect with fellow developers and mentors");
                println!("Build real-world projects and gain practical experience");
                println!();
                println!("Let's code the future together, {}!", name);
                println!("{}", "=".repeat(65));
            } else {
                println!("No name entered. Welcome anyway to iCODE ABU Zaria Bootcamp May 2025!");
            }
        }
        Err(error) => {
            println!("Error reading input: {}", error);
        }
    }

    // Optional: Pause before exit
    println!("\nPress Enter to exit...");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).unwrap();
}
