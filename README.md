Certainly! Here’s an updated version of your README with the new `--pr` functionality included:

---

# GCMGen

GCMGen (Git Commit Message Generator) is a command-line tool that generates meaningful Git commit messages and GitHub pull request content using AI models like OpenAI and Anthropic. It analyzes your staged changes (diff) and provides context-aware commit message suggestions and PR content.

## Installation

### Prerequisites

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Git**: Ensure Git is installed and accessible from the command line.

### Building the Project

```sh
git clone https://github.com/lindblomsebastian/gcmgen.git
cd gcmgen
cargo build --release
```

This builds the executable in `target/release/`.

## Usage

### Initialize API Key

Set up your API key and choose a service:

```sh
gcmgen --init
```

### Generate a Commit Message

```sh
gcmgen
```

### Generate a Commit Message with a Prefix

```sh
gcmgen -p "[WIP]"
```

### Generate a Pull Request

```sh
gcmgen --pr
```

- Retrieves the diff between your current branch and the base branch (default: `main`).
- Generates a PR title and description using the selected AI service.
- Displays the generated title and description for review.