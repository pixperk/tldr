# tldr: README Generator 🚀

Generate README.md files from your codebase using LLMs.

[![Cargo](https://github.com/pixperk/tldr/actions/workflows/cargo.yml/badge.svg)](https://github.com/pixperk/tldr/actions/workflows/cargo.yml)
[![Crates.io](https://img.shields.io/crates/v/tldr)](https://crates.io/crates/tldr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`tldr` helps you quickly generate comprehensive README.md files for your projects by leveraging the power of Large Language Models (LLMs). It analyzes your codebase, extracts relevant information, and uses a customizable prompt to create a well-structured README.md, saving you valuable time and effort.

**Problem:** Writing a good README is crucial for project adoption and understanding, but it can be a time-consuming and tedious task.

**Solution:** `tldr` automates the process by using LLMs to understand your code and generate a human-readable README.md.

**Target Audience:** Developers, software engineers, and open-source contributors who want to quickly generate README.md files for their projects.

**Use Cases:**

- Quickly create a README for a new project.
- Update an existing README with the latest changes.
- Generate READMEs for multiple projects in a consistent style.

**Key Technologies:**

- **Rust:** The core logic is written in Rust for performance and reliability.
- **Clap:** Used for command-line argument parsing.
- **Reqwest:** Used for making HTTP requests to LLM providers.
- **Serde:** Used for serializing and deserializing data.
- **Gemini & OpenAI:** Supports Google Gemini and OpenAI GPT models.

**Unique Value:**

- **Flexibility:** Supports multiple LLM providers and allows for custom prompts.
- **Streaming Mode:** Generates README sections incrementally for large projects.
- **Customizable:**  Tailor the README generation process with custom prompts and instructions.

## Features

- **README Generation:** Automatically generates a README.md file from your codebase.
- **LLM Provider Support:** Supports Google Gemini and OpenAI GPT models.
- **Custom Prompts:** Use custom prompts to tailor the README generation process.
- **Streaming Mode:** Generates README sections incrementally, useful for large projects.
- **Code Analysis:** Analyzes your codebase to extract relevant information.
- **CLI Interface:** Easy-to-use command-line interface.
- **Configuration:** Configurable through command-line arguments and environment variables.
- **Extensible:**  Designed to be extensible with support for additional LLM providers.
- **Fast Generation:** Optimized for speed and efficiency.

## Quick Start

This example shows how to generate a README using the Gemini provider.

1.  **Set your Gemini API key:**

    ```bash
    export GEMINI_API_KEY=<YOUR_GEMINI_API_KEY>
    ```

2.  **Run the command:**

    ```bash
    tldr readme --provider gemini --api-key $GEMINI_API_KEY
    ```

This will generate a `README.md` file in the current directory.

## Installation

### Prerequisites

-   Rust toolchain (recommended version: >=1.70)

### Installing `tldr`

#### From Crates.io (Recommended)

```bash
cargo install tldr
```

#### From Source

1.  Clone the repository:

    ```bash
    git clone https://github.com/pixperk/tldr.git
    cd tldr
    ```

2.  Build and install the binary:

    ```bash
    cargo install --path .
    ```

### Verification

After installation, verify that `tldr` is installed correctly by running:

```bash
tldr --version
```

This should print the version number of `tldr`.

## Usage

(Detailed usage examples will be added here)

## Architecture

(Architecture diagram and explanation will be added here)

## Configuration

(Configuration options and environment variables will be added here)

## Development

(Development setup instructions will be added here)

## Performance & Scaling

(Performance benchmarks and scaling considerations will be added here)

## Troubleshooting

(Common issues and solutions will be added here)

## Contributing

(Contribution guidelines will be added here)

## License & Legal

This project is licensed under the MIT License. See the `LICENSE` file for details.
