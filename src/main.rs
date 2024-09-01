mod anthropic;
mod cli;
mod client;
mod config;
mod gh;
mod git;
mod openai;
mod vim;

use crate::cli::build_cli;
use crate::client::{Client, CommitMessageGenerator, PullRequestGenerator};
use crate::config::{Config, ServiceConfig};
use crate::gh::create_pull_request;
use crate::git::GitError;

use crate::vim::Vim;
use inquire::{Password, Select, Text};
use std::io;
use std::io::Write;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = build_cli();

    if matches.get_flag("init") {
        let services = vec!["OpenAI", "Anthropic"];
        let selected_service = Select::new("Choose your AI service:", services).prompt()?;

        let default_models = match selected_service {
            "OpenAI" => "gpt-4o-mini",
            "Anthropic" => "claude-3-5-sonnet-20240620",
            &_ => unreachable!("No other services implement yet."),
        };

        // Prompt user for the model name
        let model = Text::new("Enter the model name (or the default values will be used):")
            .with_initial_value(default_models)
            .prompt()?;

        // Prompt user for the API key (secret input)
        let api_token = Password::new("Enter your API key:")
            .with_display_mode(Password::DEFAULT_DISPLAY_MODE)
            .prompt()?;

        // Construct the service configuration
        let service_config = ServiceConfig { api_token, model };

        // Load existing configuration or create a new one
        let mut config = match Config::load() {
            Ok(config) => config,
            Err(_) => Config {
                default_service: selected_service.to_string(),
                services: std::collections::HashMap::new(),
            },
        };

        // Update the configuration with the chosen service
        config
            .services
            .insert(selected_service.to_string(), service_config);
        config.default_service = selected_service.to_string();

        // Save the configuration
        config.save()?;
        println!("Configuration saved successfully.");
        return Ok(());
    }

    // Load the API key from config
    let mut config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Error: No config file found. Please run 'gcmgen --init to initialize.");
            exit(1);
        }
    };

    if matches.get_flag("list-services") {
        println!("{}", config);
        return Ok(());
    }

    if let Some(service) = matches.get_one::<String>("set-default") {
        match config.set_default_service(service) {
            Ok(_) => {
                println!("Default service set to '{}'.", service);
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error setting default service: {}", e);
                exit(1);
            }
        }
    }

    let client = Client::new(
        config.get_default_service_config().unwrap(),
        &config.default_service,
    )?;

    let prefix = matches.get_one::<String>("prefix");

    if matches.get_flag("pull-request") {
        loop {
            // Get the diff from Git
            let branch_diff = match git::get_branch_diff(Some("main")) {
                Ok(branch_diff) => branch_diff,
                Err(GitError::EmptyDiff) => {
                    eprintln!("Error: {}", GitError::EmptyDiff);
                    return Ok(()); // Not an actual error, just exit gracefully
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Err(Box::new(e));
                }
            };
            let title = client.generate_pr_title(&branch_diff, prefix)?;
            let description = client.generate_pr_description(&branch_diff)?;

            println!("\nGenerated PR Title:\n{}\n", title);
            println!("Generated PR Description:\n{}\n", description);

            // Ask the user what they want to do
            print!("Do you want to (a)ccept, (r)egenerate, or (c)ancel? [a/e/c]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "a" | "A" => {
                    // Open the PR in the web browser with the title and description
                    create_pull_request(&title, &description, Some("main"))?;
                    println!("Pull request created successfully.");
                    return Ok(());
                }
                "r" | "R" => {
                    println!("Regenerating commit message...");
                }
                "c" => {
                    // Cancel the PR creation process
                    println!("PR creation canceled.");
                    return Ok(());
                }
                _ => {
                    // Invalid input, ask again
                    println!("Invalid option. Please choose 'a' to accept, 'e' to edit, or 'c' to cancel.");
                }
            }
        }
    }

    loop {
        // Get the diff from Git
        let diff = match git::get_diff() {
            Ok(diff) => diff,
            Err(GitError::EmptyDiff) => {
                eprintln!("Error: {}", GitError::EmptyDiff);
                return Ok(()); // Not an actual error, just exit gracefully
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(Box::new(e));
            }
        };

        let mut commit_message = client.generate_commit_message(&diff, prefix)?;

        // Display the generated commit message to the user
        println!("\nGenerated commit message:\n\n{}\n", commit_message);

        // Ask the user what they want to do
        print!("Do you want to (a)ccept, (e)dit, (r)egenerate, or (q)uit? If you quit, nothing will be committed [a/r/q]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "a" | "A" => {
                // Accept the commit message and commit the changes
                git::commit(&commit_message)?;
                println!("Committed with message: {}", commit_message);
                return Ok(());
            }
            "e" | "E" => {
                commit_message = Vim::new().edit_message(&commit_message)?;
                git::commit(&commit_message)?;
                println!("Committed with edited message: {}", commit_message);
                return Ok(());
            }
            "r" | "R" => {
                // Regenerate the commit message (the loop will run again)
                println!("Regenerating commit message...");
            }
            "q" | "Q" => {
                // Skip the commit process
                println!("Commit skipped.");
                return Ok(());
            }
            _ => {
                // Invalid input, ask again
                println!("Invalid option. Please choose 'a' to accept, 'r' to regenerate, or 'q' to quit.");
            }
        }
    }
}
