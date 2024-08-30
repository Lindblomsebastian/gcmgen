Certainly! Here’s a more concise version of the README, focusing on the essential information:

---

# GCMGen

GCMGen (Git Commit Message Generator) is a command-line tool that generates meaningful Git commit messages using AI models like OpenAI and Anthropic. It analyzes your staged changes (diff) and provides context-aware commit message suggestions.

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

- Retrieves the diff of your staged changes.
- Sends the diff to the selected AI service.
- Prompts you to accept, regenerate, or skip the commit message.