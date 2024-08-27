use std::io;

use std::process::Command;

pub fn get_diff() -> Result<String, io::Error> {
    Command::new("git")
        .args(&["diff", "--staged"])
        .output()
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to get git diff",
                ))
            }
        })
}

pub fn commit(message: &str) -> Result<(), io::Error> {
    Command::new("git")
        .args(&["commit", "-m", message])
        .status()
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to commit changes",
                ))
            }
        })
}
