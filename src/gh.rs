use std::error::Error;
use std::process::Command;

#[allow(dead_code)]
fn create_pull_request(
    title: &str,
    description: &str,
    base_branch: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let branch = base_branch.unwrap_or("main"); // main is default

    let mut command = Command::new("gh");

    command.args([
        "pr",
        "create",
        "--title",
        title,
        "--body",
        description,
        "--base",
        branch,
        "--web",
    ]);

    let status = command.status()?;

    if status.success() {
        println!("Pull request creation page opened in your browser.");
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to create pull request",
        )))
    }
}
