use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::Command;
use tempfile::NamedTempFile;

pub struct Vim;

impl Vim {
    pub fn new() -> Self {
        Vim
    }

    pub fn edit_message(&self, message: &str) -> Result<String, Box<dyn Error>> {
        // Create a temporary file with the initial commit message
        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "{}", message)?;

        // Get the path of the temporary file
        let temp_file_path = temp_file.path().to_str().unwrap().to_string();

        // Open the temporary file in Vim
        Command::new("vim").arg(temp_file_path.clone()).status()?;

        // Read the edited commit message back from the file
        let mut edited_message = String::new();
        OpenOptions::new()
            .read(true)
            .open(temp_file_path)?
            .read_to_string(&mut edited_message)?;

        Ok(edited_message.trim().to_string())
    }
}
