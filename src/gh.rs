use std::error::Error;
use std::process::Command;

#[allow(dead_code)]
fn create_pull_request(
    title: &str,
    description: &str,
    base_branch: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("gh");

    command.args([
        "pr",
        "create",
        "--title",
        title,
        "--body",
        description,
        "--web",
    ]);

    if let Some(branch) = base_branch {
        command.arg("--base").arg(branch);
    }

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
