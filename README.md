
# TLDR: AI-Powered README Generator [![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://example.com/build) [![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://example.com/version) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`tldr` is a CLI tool that leverages LLMs to automatically generate comprehensive README.md files for your projects, using code analysis and Retrieval-Augmented Generation (RAG).

## ✨ Overview

The `tldr` tool addresses the common problem of creating high-quality README documentation for software projects.  Manually writing READMEs can be time-consuming and inconsistent.  `tldr` automates this process by analyzing source code, extracting key information, and using a Large Language Model (LLM) to generate a well-structured and informative README.md file.

The target audience for this tool includes:

*   Software developers who want to quickly generate professional-looking READMEs.
*   Open-source maintainers who want to improve the discoverability and usability of their projects.
*   Teams looking to standardize documentation practices.

Key technologies used include:

*   **Rust:**  The primary programming language, offering performance and reliability.
*   **Clap:** Used for command-line argument parsing.
*   **Reqwest:** An asynchronous HTTP client for interacting with LLM APIs.
*   **Serde:** A serialization/deserialization framework for handling JSON data.
*   **Tokio:** An asynchronous runtime for efficient I/O operations.
*   **Walkdir:** For recursively traversing directory structures.
*   **Indicatif:** For displaying progress bars during code analysis.
*   **Gemini/OpenAI:** The LLM providers that power the README generation.

What makes `tldr` unique is its integration of code analysis, embeddings (implicit through LLM understanding of code), and RAG to produce contextually relevant and comprehensive documentation. The tool supports both streaming and non-streaming modes, allowing for real-time feedback and faster generation times.

## ✨ Features

*   **README Generation:** Automatically generates a comprehensive README.md file for software projects.
*   **LLM Powered:** Leverages Large Language Models (LLMs) like Gemini and OpenAI for content creation.
*   **Customizable LLM Provider:** Supports Gemini and OpenAI as LLM providers, allowing users to choose based on their needs and API keys.
*   **Streaming Mode:** Generates README sections in streaming mode, providing real-time feedback and reducing wait times.
*   **CLI Interface:** Provides a command-line interface (CLI) for easy interaction and automation.
*   **Project Directory Specification:** Allows users to specify the path to the project directory for README generation.
*   **API Key Authentication:** Supports API key authentication for accessing LLM services.
*   **Configuration via `.env`:** Uses `dotenvy` to load environment variables from a `.env` file.
*   **Asynchronous Operations:** Uses `tokio` for asynchronous operations, improving performance and responsiveness.
*   **HTTP Client:** Utilizes `reqwest` for making HTTP requests to LLM APIs.
*   **Error Handling:** Implements error handling to gracefully manage potential issues during README generation.
*   **Extensible Architecture:** Modular design with separate modules for LLM clients, prompts, and utilities, facilitating future extensions and modifications.
*   **Comprehensive Code Analysis:** Supports a wide range of programming languages, thanks to an extensive list of file extensions.
*   **Progress Indicators:** Provides visual progress updates during code analysis and README generation using `indicatif`.

## 🚀 Quick Start

Generate a README for the current directory using the default Gemini provider:

```bash
tldr readme --path .
```

**Expected Output:**

After running the command, a `README.md` file will be created in the current directory.  The console output will indicate the progress of the README generation process. Example:

```text
🚀 Generating README with Gemini (fast mode)...
🎉 README.md generated successfully at: ./README.md
```

## 🛠️ Installation

### Installation
1.  **Prerequisites:**
    *   **Rust Toolchain:** Ensure you have the Rust toolchain installed. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).  A recent version of Rust (>= 1.70) is recommended.
    *   **API Key:** You will need an API key for either Google Gemini or OpenAI. Obtain one from the respective provider's website.  Set the appropriate environment variable (`GEMINI_API_KEY` or `OPENAI_API_KEY`) or pass it via the `--api-key` flag.
2.  **Clone the Repository:**
    ```bash
    git clone <repository_url>
    cd tldr
    ```
3.  **Build the Project:**
    ```bash
    cargo build --release
    ```
    This command compiles the project in release mode, optimizing for performance. The executable will be located in `target/release/tldr`.
4.  **Add to Path (Optional):**
    To make the `tldr` executable accessible from any directory, add the `target/release` directory to your system's PATH environment variable.
    *   **Linux/macOS:**
        Edit your `~/.bashrc` or `~/.zshrc` file and add the following line:
        ```bash
        export PATH="$PATH:/path/to/tldr/target/release"
        ```
        Replace `/path/to/tldr` with the actual path to the project directory.  Then, source the file:
        ```bash
        source ~/.bashrc  # or source ~/.zshrc
        ```
    *   **Windows:**
        *   Search for "Edit environment variables for your account" in the Start Menu.
        *   Edit the "Path" variable (either user or system).
        *   Add a new entry with the path to the `target/release` directory.
5.  **Verification:**
    After installation, verify that `tldr` is correctly installed by running:
    ```bash
    tldr --help
    ```
    This should display the command-line help information for the `tldr` tool.

## ⚙️ Usage

### Usage
The primary way to interact with `tldr` is through its command-line interface. Below are examples demonstrating how to generate a `README.md` file for your project.

**Basic Usage:**
```bash
tldr readme --path <project_path>
```
This command generates a `README.md` for the project at the specified path, using the default Gemini LLM provider.  The Gemini API key is expected to be in the `GEMINI_API_KEY` environment variable or a `.env` file. If `--path` is not provided, the current directory is used.

**Using OpenAI:**
```bash
tldr readme --provider openai --path <project_path> --api-key <your_openai_api_key>
```
This command generates a `README.md` using the OpenAI LLM provider. You **must** provide your OpenAI API key using the `--api-key` argument. Replace `<your_openai_api_key>` with your actual API key.

**Streaming Mode:**
```bash
tldr readme --path <project_path> --streaming
```
This command generates the `README.md` and streams the output to the console as it's being generated, providing real-time feedback.

**Example using a `.env` file**

Create a `.env` file in the root of your project and add your API key:
```
GEMINI_API_KEY=YOUR_GEMINI_API_KEY
```
Then, run the basic usage command:
```bash
tldr readme --path .
```

**Available Options:**

| Option         | Short | Long      | Description                                                                                                               | Default Value |
|----------------|-------|-----------|---------------------------------------------------------------------------------------------------------------------------|---------------|
| `path`         |       |           | Path to the project directory.                                                                                            | `.`           |
| `provider`     | `-p`  | `--provider`| LLM provider to use (Gemini or OpenAI).                                                                                    | `gemini`      |
| `api_key`      | `-k`  | `--api-key` | API key for the selected LLM provider. If not provided, the tool looks for an environment variable (e.g., `GEMINI_API_KEY`). |               |
| `streaming`    | `-s`  | `--streaming`| Use streaming mode for generating the README.                                                                                 | `false`       |

## 🏛️ Architecture

The `tldr` tool is designed with a modular architecture to facilitate maintainability and extensibility.  Here's a high-level overview:

1.  **CLI Parsing:** The `cli.rs` module defines the command-line interface using the `clap` crate. It parses user arguments and dispatches the appropriate command.
2.  **Code Collection:** The `rag.rs` module is responsible for collecting source code chunks from the specified project directory.  It recursively traverses the directory structure, identifies code files based on their extensions (defined in `util/ext.rs`), and reads their contents into strings.  The `indicatif` crate provides a progress bar during this process.
3.  **LLM Abstraction:** The `llm` module provides an abstraction layer for interacting with different LLM providers (Gemini and OpenAI).
    *   `llm/client.rs`: Defines the `LlmClient` struct, which manages the HTTP client and API key for making requests to the LLM APIs.
    *   `llm/provider.rs`: Defines the `LlmGenerator` enum, which encapsulates the logic for interacting with different LLM providers. It selects the appropriate generator based on the user's choice.
    *   `llm/gemini.rs` and `llm/openai.rs`: Implement the specific logic for interacting with the Gemini and OpenAI APIs, respectively. They handle request formatting, response parsing, and error handling.
    *   `llm/prompt.rs`: Contains the system prompt used to guide the LLM in generating the README.
4.  **README Generation:** The `util/readme.rs` module orchestrates the README generation process.  It receives the collected code chunks and the LLM generator, calls the generator to create the README content (either in streaming or non-streaming mode), and writes the content to the `README.md` file.
5.  **Asynchronous Execution:**  The entire process is executed asynchronously using the `tokio` runtime.

**Data Flow:**

1.  User executes the `tldr` command with specified arguments.
2.  `clap` parses the arguments and determines the desired action (e.g., generate README).
3.  `rag.rs` collects code chunks from the project directory.
4.  `llm/provider.rs` selects the appropriate LLM generator based on the user's choice (Gemini or OpenAI).
5.  The selected LLM generator sends a request to the LLM API with the collected code chunks and the system prompt.
6.  The LLM generates the README content.
7.  If streaming mode is enabled, the content is streamed to the console and written to the `README.md` file incrementally.  Otherwise, the entire content is generated first, then written to the file.
8.  The `README.md` file is created or updated in the project directory.

## ⚙️ Configuration

### Environment Variables

The following environment variables can be used to configure the `tldr` tool:

| Variable        | Description                                                                            | Default Value |
|-----------------|----------------------------------------------------------------------------------------|---------------|
| `GEMINI_API_KEY`| API key for the Google Gemini LLM. Required if using Gemini and `--api-key` is omitted. |               |
| `OPENAI_API_KEY`| API key for the OpenAI LLM. Required if using OpenAI and `--api-key` is omitted.   |               |

To set an environment variable, you can use the following command in your terminal (Linux/macOS):

```bash
export GEMINI_API_KEY="your_gemini_api_key"
```

Or, you can define the variables in a `.env` file in the root of your project:

```
GEMINI_API_KEY=your_gemini_api_key
```

### Configuration file

Currently, there is no support for a config file. All configuration is performed by environment variables or CLI flags.

## 🧑‍💻 Development

### Setup Instructions

1.  **Install Rust:** Ensure you have a recent version of Rust installed (>= 1.70).  Follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  **Clone the Repository:**
    ```bash
    git clone <repository_url>
    cd tldr
    ```

### Build Process

To build the project, run the following command:

```bash
cargo build
```

To build the project in release mode (optimized for performance), run:

```bash
cargo build --release
```

### Testing

Currently, there are no automated tests.

### Code Organization

*   `src/main.rs`: Entry point of the application.  Parses command-line arguments and dispatches the appropriate command.
*   `src/cli.rs`: Defines the command-line interface using the `clap` crate.
*   `src/rag.rs`: Implements the Retrieval-Augmented Generation (RAG) logic, collecting code chunks from the project directory.
*   `src/llm/`: Contains modules related to interacting with Large Language Models (LLMs).
    *   `src/llm/client.rs`: Defines the HTTP client for interacting with LLM APIs.
    *   `src/llm/provider.rs`: Defines the LLM provider abstraction.
    *   `src/llm/gemini.rs`: Implements the logic for interacting with the Gemini API.
    *   `src/llm/openai.rs`: Implements the logic for interacting with the OpenAI API.
    *   `src/llm/prompt.rs`: Contains the system prompt used to guide the LLM.
*   `src/util/`: Contains utility modules.
    *   `src/util/ext.rs`: Defines the list of supported code file extensions.
    *   `src/util/readme.rs`: Implements the logic for generating the README file.

### Code Conventions

*   Use `rustfmt` to format your code.
*   Follow the Rust API Guidelines.
*   Write clear and concise code with meaningful variable and function names.
*   Add comments to explain complex logic.
*   Handle errors gracefully.

## ⚡ Performance & Scaling

The performance of `tldr` is primarily limited by the LLM API's response time and the number of code files in the project.

*   **Resource Requirements:** The tool requires a moderate amount of memory to store the code chunks and the generated README content. The CPU usage is relatively low, as most of the processing is done by the LLM API.
*   **Scaling Considerations:** For very large projects with thousands of code files, consider increasing the timeout duration for the HTTP client to avoid connection errors. You might also need to optimize the code chunking strategy to reduce the amount of data sent to the LLM API.

## 🐛 Troubleshooting

### Common Issues and Solutions

*   **API Key Errors:** Ensure that you have provided a valid API key for the selected LLM provider, either through the `--api-key` flag or the appropriate environment variable.
*   **Connection Errors:** Check your internet connection and ensure that the LLM API is accessible. You might also need to increase the timeout duration for the HTTP client if you are experiencing frequent connection errors.
*   **Rate Limiting:** LLM providers typically have rate limits. If you are exceeding the rate limit, you might need to implement retry logic with exponential backoff.
*   **Empty README:** If the generated README is empty or incomplete, try adjusting the system prompt or providing more context to the LLM.

### Debug/Logging Configuration

Currently, there is no support for custom logging configuration.

### Known Limitations

*   The quality of the generated README depends on the quality of the code and the system prompt.
*   The tool may not be able to handle very large projects with thousands of code files efficiently.
*   There are no automated tests.

### Where to Get Help

If you encounter any issues or have questions about using `tldr`, please open an issue on the GitHub repository.

## 🤝 Contributing

Contributions are welcome! Here's a summary of how to contribute:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Implement your changes, following the code conventions and guidelines.
4.  Add tests for your changes.
5.  Commit your changes with clear and concise commit messages.
6.  Push your branch to your forked repository.
7.  Submit a pull request to the main repository.

**Development Workflow:**

1.  Create a new branch: `git checkout -b feature/your-feature-name`
2.  Implement your changes.
3.  Test your changes (currently manual).
4.  Commit your changes: `git commit -m "feat: Add your feature"`
5.  Push your branch: `git push origin feature/your-feature-name`
6.  Submit a pull request.

**Code Standards and Review Process:**

*   All code must be formatted using `rustfmt`.
*   Pull requests will be reviewed by the project maintainers.
*   Changes may be requested before a pull request is merged.

**Issue Reporting Guidelines:**

*   Provide a clear and concise description of the issue.
*   Include steps to reproduce the issue.
*   Include the version of `tldr` you are using.
*   Include any relevant error messages or logs.

## 📄 License & Legal

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Third-party dependencies and their licenses:**

This project uses several third-party dependencies, each with its own license. Please refer to the `Cargo.toml` file for a complete list of dependencies and their respective licenses.
