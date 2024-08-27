mod cli;
mod config;
mod git;
mod openai;

use crate::cli::build_cli;
use crate::config::Config;
use crate::openai::OpenAIClient;
use std::io::Write;
use std::{io, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = build_cli();

    if let Some(token) = matches.get_one::<String>("init") {
        Config::save_token(token)?;
        println!("API token saved successfully.");
        return Ok(());
    }

    // Load the API key from config
    let api_key = match Config::load_token() {
        Ok(key) => key,
        Err(_) => {
            eprintln!(
                "Error: API token not found. Please run 'gcm --init <your_token>' to initialize."
            );
            process::exit(1);
        }
    };

    // Initialize the OpenAI client with the default model or a specified one
    let mut openai_client = OpenAIClient::new(&api_key);
    if let Some(model) = matches.get_one::<String>("model") {
        openai_client = openai_client.with_model(model);
    }

    // Get the diff from Git
    let _diff = git::get_diff()?;

    loop {
        // Get the diff from Git
        let diff = git::get_diff()?;

        // Generate the commit message
        let commit_message = openai_client.generate_commit_message(&diff)?;

        // Display the generated commit message to the user
        println!("\nGenerated commit message:\n\n{}\n", commit_message);

        // Ask the user what they want to do
        print!("Do you want to (a)ccept, (r)egenerate, or (q)uit? If you quit, nothing will be committed [a/r/q]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "a" | "A" => {
                // Accept the commit message and commit the changes
                git::commit(&commit_message)?;
                println!("Committed with message: {}", commit_message);
                break;
            }
            "r" | "R" => {
                // Regenerate the commit message (the loop will run again)
                println!("Regenerating commit message...");
            }
            "q" | "Q" => {
                // Skip the commit process
                println!("Commit skipped.");
                break;
            }
            _ => {
                // Invalid input, ask again
                println!("Invalid option. Please choose 'a' to accept, 'r' to regenerate, or 'q' to quit.");
            }
        }
    }

    Ok(())
}
