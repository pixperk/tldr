```markdown
# tldr-gen: Generate Documentation with LLMs 📚✨

[![Build Status](https://img.shields.io/github/actions/workflow/status/your-username/tldr-gen/main.yml?branch=main)](https://github.com/your-username/tldr-gen/actions)
[![Version](https://img.shields.io/crates/v/tldr-gen)](https://crates.io/crates/tldr-gen)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`tldr-gen` automatically generates documentation from your codebase using Large Language Models (LLMs).

## Overview

`tldr-gen` simplifies documentation by leveraging LLMs. Instead of manually writing documentation, this tool analyzes your codebase and automatically creates a `README.md` file, saving developers time and ensuring documentation stays up-to-date with code changes.

**Target Audience:**

*   Software developers
*   Technical writers
*   Open-source maintainers

**Key Technologies:**

*   Rust
*   `clap` for CLI argument parsing
*   `reqwest` for HTTP requests to LLM APIs
*   `dotenvy` for environment variable loading

**What Makes it Unique:**

`tldr-gen` focuses on ease of use and integration. A single command generates a comprehensive README, reducing documentation friction. It's designed to be extensible and adaptable to different LLM providers and codebase structures, though currently relies on specific assumptions about LLM API key configuration through environment variables.

## Features

*   **Automated README Generation:** Analyzes codebase and generates a `README.md` file.
*   **CLI Interface:** Simple command-line interface for easy usage.
*   **LLM Powered:** Uses Large Language Models to understand and document your code.
*   **Customizable Path:** Specify the project directory to generate documentation for.
*   **Error Handling:** Provides informative error messages for troubleshooting.

## Quick Start

This example shows how to generate a `README.md` file for your project.

1.  **Install `tldr-gen`** (See Installation section below).

2.  **Set your LLM API key** as an environment variable (e.g., `OPENAI_API_KEY`). This is required for the LLM to function correctly.

3.  **Run the command:**

    ```bash
    tldr readme
    ```

    This will generate a `README.md` file in your current directory.

## Installation

### Prerequisites

*   Rust toolchain (install via [rustup](https://rustup.rs/))

### Install via Cargo

```bash
cargo install tldr-gen
```

This installs the `tldr-gen` executable to your `~/.cargo/bin` directory. Ensure this directory is in your `PATH`.

### Verify Installation

```bash
tldr --version
```

This should print the installed version of `tldr-gen`.

## Usage

The `tldr-gen` CLI provides a simple interface for generating README files.

### Generating a README

```bash
tldr readme --path <project_directory>
```

*   `readme`:  Specifies the "readme" subcommand for README generation.
*   `--path <project_directory>`: Specifies the path to the project directory. Defaults to the current directory (`.`) if not provided.

**Examples:**

To generate a `README.md` file for a project in the `my_project` directory:

```bash
tldr readme --path my_project
```

To generate the README in the current directory:

```bash
tldr readme
```

**Important:**

*   Ensure your LLM API key is set as an environment variable (e.g., `OPENAI_API_KEY`). `tldr-gen` relies on this environment variable being set.
*   The generated `README.md` file overwrites any existing file with the same name in the specified directory.

## Architecture

`tldr-gen` operates with the following components:

1.  **CLI Parser:** The `cli.rs` module uses `clap` to parse command-line arguments.
2.  **Project Analyzer:** (Currently implicit) Analyzes the code within the given path. This part of the project is currently underdeveloped, but the tool has the potential to be expanded by extracting relevant code metadata.
3.  **LLM Client:** The `llm/client.rs` module handles communication with the LLM API. It initializes a `reqwest` HTTP client and manages the API key.
4.  **README Generator:** (Within `src/util/readme.rs` - inferred)  Constructs the `README.md` content based on the analysis and LLM responses.

The overall process involves:

1.  Parsing command-line arguments to get the project path.
2.  (Future Improvement) Analysing the project's source code.
3.  Communicating with the configured LLM to generate documentation snippets.
4.  Combining documentation snippets into a complete `README.md` file and writing it to the project directory.

## Configuration

`tldr-gen` relies on environment variables for configuration:

*   `OPENAI_API_KEY`: (Example) This environment variable is crucial; `tldr-gen` expects an LLM API key to be set for documentation generation to work correctly.
    *   Description: The API key for your LLM provider.
    *   Default: None (required).
    *   Example: `sk-your-api-key`

You can set environment variables in your `.env` file or directly in your shell.

```bash
# .env file
OPENAI_API_KEY=sk-your-api-key
```

```bash
export OPENAI_API_KEY=sk-your-api-key # setting the api key in bash
```

## Development

### Setup

1.  Clone the repository:

    ```bash
    git clone https://github.com/your-username/tldr-gen.git
    cd tldr-gen
    ```

2.  Install Rust dependencies:

    ```bash
    cargo build
    ```

### Build

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Code Conventions

*   Follow Rust's style guidelines (use `cargo fmt`).
*   Write clear and concise code.
*   Add comments to explain complex logic.

## Performance & Scaling

The performance of `tldr-gen` depends largely on the size of the codebase and the response time of the LLM API. The tool's HTTP client has a timeout of 120 seconds to allow enough time for LLM processing.

## Troubleshooting

*   **Error: API Key not found:** Ensure the `OPENAI_API_KEY` environment variable is set correctly.
*   **Error: Request Timeout:** Check your internet connection and the status of the LLM API.  Consider increasing the timeout duration if necessary.
*   **Error: Generated README is incomplete/incorrect:** This may indicate a limitation in the LLM's understanding of the code. Consider manually editing the generated README or providing more context to the LLM.

For further assistance, create an issue on the [GitHub repository](https://github.com/your-username/tldr-gen/issues).

## Contributing

Contributions are welcome! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Write tests for your changes.
4.  Submit a pull request.

## License & Legal

This project is licensed under the MIT License. See the `LICENSE` file for details.
```