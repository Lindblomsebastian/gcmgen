# gcmgen

GCMGen (Git Commit Message Generator) is a command-line tool that generates meaningful Git commit messages using OpenAI's language models. It automates the process of writing commit messages by analyzing your staged changes (diff) and providing concise, context-aware suggestions.

## Features

- **AI-Powered Commit Messages**: Generate commit messages based on the staged changes in your Git repository.
- **Model Flexibility**: Use a default OpenAI model or specify a different one.
- **Interactive Mode**: Review the generated commit message, regenerate if necessary, or skip the commit process entirely.

## Installation

### Prerequisites

- **Rust**: Make sure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Git**: Git should be installed and accessible from the command line.

### Cloning the Repository

```sh
git clone https://github.com/lindblomsebastian/gcmgen.git
cd gcmgen
```

### Building the Project

Compile the project using Cargo:

```sh
cargo build --release
```

This will create an executable in the `target/release/` directory.

### Running the Program

You can run the program directly from the command line:

```sh
cargo run -- --init your_openai_token
```

Or use the built executable:

```sh
./target/release/gcmgen --init your_openai_token
```

## Usage

### Initializing the OpenAI API Token

Before you can generate commit messages, you need to initialize your OpenAI API token:

```sh
gcmgen --init your_openai_token
```

This command saves your token locally, so you don't need to enter it each time.

### Generating a Commit Message

To generate a commit message based on your staged changes:

```sh
gcmgen
```

This will:

1. Retrieve the diff of your staged changes.
2. Send the diff to OpenAI to generate a commit message.
3. Prompt you to accept, regenerate, or skip the commit.

### Specifying a Model

You can specify a different OpenAI model using the `--model` option:

```sh
gcmgen --model text-davinci-003
```

The default model is `gpt-4o-mini`.

## Error Handling

- **Empty Stage**: If no files are staged, the program will display an error and exit gracefully.
- **API Key Missing**: If the API key is missing, the program will prompt you to initialize it with `--init`.
