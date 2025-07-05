# tldr - Gemini-Powered README Generator ✨

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`tldr` automatically generates README.md files using Gemini, embeddings, and RAG.

## Overview

`tldr` is a command-line tool designed to streamline the process of creating high-quality README files for Rust projects. By leveraging Google's Gemini 2.0 LLM, code embeddings, and Retrieval Augmented Generation (RAG), `tldr` automates documentation, saving developers time and ensuring consistent, comprehensive project information.

This tool targets Rust developers who want to easily generate a README.md file for their project with minimal manual effort. It's particularly useful for projects lacking initial documentation, or for keeping existing READMEs up-to-date as the codebase evolves.

`tldr` uses Rust, the Clap crate for CLI argument parsing, the Dotenvy crate for reading the .env files, the Requests crate for sending HTTP requests to Gemini, the Serde crate for serialization, and the Walkdir crate to traverse through the file system to collect code chunks. The main architectural approach is to read files from a project, collect them into code chunks and then send those code chunks to the Gemini LLM to generate the README.md content.

`tldr` stands out by combining cutting-edge LLM technology with code-aware embeddings to create more insightful and relevant documentation, compared to simpler template-based or manual approaches.

## Features

### Core Functionality

*   **Automated README Generation:** Automatically creates `README.md` files based on your codebase.
*   **Retrieval Augmented Generation (RAG):** Employs RAG to fetch relevant code snippets and context, ensuring comprehensive documentation.
*   **Embeddings (Planned):** Uses embeddings of code to capture semantic meaning and improve documentation accuracy. (Currently a placeholder).
*   **LLM Powered:** Uses Google's Gemini 2.0 to generate high-quality, context-aware documentation.

### User Experience
*   **Simple CLI:** Easy-to-use command-line interface with intuitive options.
*   **Progress Indicators:** Displays progress updates during code scanning and documentation generation.
*   **Configurable Project Path**: Allows specifying a custom project path for document generation.

### Technology
*   **Extensive Code Extension Support**: Supports a wide array of code extensions (175+).
*   **MIT License:** Open-source and freely available for use and modification.

## Quick Start

Generate a `README.md` for your current directory with:

```bash
tldr readme
```

Expected Output:

```text
🔍 Scanning codebase...
📄 Processing: src\main.rs
📄 Processing: src\rag.rs
📄 Processing: src\util\mod.rs
📄 Processing: src\util\ext.rs
📄 Processing: src\util\readme.rs
📄 Processing: src\llm\mod.rs
📄 Processing: src\llm\client.rs
📄 Processing: src\llm\gemini.rs
📄 Processing: src\llm\prompt.rs
📄 Processing: src\cli.rs
✅ Processed 10 files, found 10 code files
Generating README with Gemini AI...
Processing response...
✅ README.md generated successfully at: ./README.md
```

## Installation

Ensure you have Rust and Cargo installed on your system. Then, install `tldr` from crates.io:

```bash
cargo install tldr
```

### System Requirements

*   Rust toolchain (stable version recommended)
*   Cargo package manager

### Prerequisites

Before using `tldr`, ensure you have:

*   A Google Cloud project with the Gemini API enabled.
*   A Gemini API key. Set this as an environment variable named `GEMINI_API_KEY`.
*   (Optional) A `.env` file at the root of your project. This file can be used to set the `GEMINI_API_KEY` if you don't want to set it globally.

## Usage

### Generating README.md

To generate a `README.md` file in your project directory, navigate to the directory in your terminal and run:

```bash
tldr readme
```

To specify a different project path, use the `--path` option:

```bash
tldr readme --path /path/to/your/project
```

### Command-Line Options

The `tldr readme` subcommand accepts the following options:

*   `--path <PATH>`: Specifies the path to the project directory. Defaults to the current directory (`.`).

## Architecture

`tldr` operates using a Retrieval Augmented Generation (RAG) architecture. The key components are:

1.  **CLI Interface (cli.rs):** Handles user input and command parsing using the `clap` crate.
2.  **Code Chunk Collection (rag.rs):** Scans the project directory, identifies code files based on their extension (using ext.rs), and extracts their content.  This forms the "chunks".
3.  **Embedding (embed.rs):** Creates vector embeddings of the code chunks (Placeholder). This is intended to capture the semantic meaning of the code.
4.  **LLM Interaction (llm/):**
    *   **LLM Client (llm/client.rs):** Manages communication with the Gemini API using the `reqwest` crate.
    *   **Gemini Integration (llm/gemini.rs):** Formats the code chunks and a system prompt (prompt.rs) into a prompt for the Gemini model, sends the request, and processes the response.
5.  **README Generation (util/readme.rs):** Takes the generated README content from Gemini and saves it to a `README.md` file in the project directory.

The data flow is as follows:

`CLI -> Code Chunk Collection -> Embedding (TODO) -> LLM Interaction -> README Generation`

## Configuration

### Environment Variables

| Variable        | Description                                     | Default |
| --------------- | ----------------------------------------------- | ------- |
| `GEMINI_API_KEY` | Your Google Gemini API key.  Required for LLM. |         |

## Development

If you'd like to contribute to `tldr`, follow these steps to set up your development environment:

### Prerequisites

*   Rust toolchain (stable)
*   Git

### Setup

1.  Clone the repository:

    ```bash
    git clone https://github.com/<your_username>/tldr.git
    cd tldr
    ```

2. Build the project

    ```bash
    cargo build
    ```

### Testing

Run tests using:
```bash
cargo test
```

## Performance & Scaling

`tldr`'s performance primarily depends on:

*   **Codebase Size**: Larger codebases take longer to scan.
*   **LLM API Latency**: Generating the README content depends on the speed of the Gemini API.
*   **Internet Connection Speed**: A stable and fast internet connection is required for making requests to the Gemini API.

As the tool currently only makes a single API call, the scaling considerations are limited.  Future work may involve parallelizing code chunk processing for larger codebases.

## Troubleshooting

*   **API Errors**: Ensure your `GEMINI_API_KEY` is valid and the Gemini API is enabled in your Google Cloud project.
*   **Slow Generation:** This is expected due to the LLM processing time.
*   **Incomplete Documentation:** The quality of the generated README depends on the quality and clarity of the comments and code in your project.

If you encounter any issues, please create an issue in the repository or contact the maintainers.

## Contributing

Contributions are greatly appreciated! To contribute:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Implement your changes.
4.  Submit a pull request.

Please adhere to the project's code style and contribution guidelines.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
