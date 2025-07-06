# 📝 tldrs

> **TL;DR for your codebase** - Automatically generate comprehensive README.md files using AI

[![Crates.io](https://img.shields.io/crates/v/tldrs)](https://crates.io/crates/tldrs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/workflow/status/pixperk/tldrs/CI)](https://github.com/pixperk/tldrs/actions)

**tldrs** is a powerful command-line tool that automatically generates professional README.md files by analyzing your codebase using Large Language Models (LLMs). Save hours of documentation work and ensure consistency across your projects.

## ✨ Features

- 🤖 **AI-Powered Analysis** - Leverages Google Gemini and OpenAI GPT models
- 📂 **Smart Codebase Scanning** - Intelligently analyzes project structure and dependencies
- 🎨 **Custom Prompts** - Use your own prompts or load from files for tailored documentation
- ⚡ **Streaming Mode** - Watch your README generate in real-time, section by section
- 🔧 **Multiple LLM Providers** - Switch between Gemini and OpenAI based on your preference
- 📝 **Professional Templates** - Generates comprehensive, well-structured documentation
- 🎯 **Additional Instructions** - Fine-tune output with custom instructions
- 🚀 **Fast & Efficient** - Optimized for speed with smart content filtering

## 🚀 Quick Start

### Installation

Install tldrs from crates.io:

```bash
cargo install tldrs
```

### Basic Usage

1. **Set up your API key** (choose one):
   ```bash
   # For Google Gemini (recommended - free tier available)
   export GEMINI_API_KEY="your-gemini-api-key"
   
   # For OpenAI
   export OPENAI_API_KEY="your-openai-api-key"
   ```

2. **Generate a README**:
   ```bash
   # In your project directory
   tldrs readme
   
   # Or specify a path
   tldrs readme /path/to/your/project
   ```

3. **Your README.md is ready!** 🎉

## 📖 Detailed Usage

### Command Reference

```bash
tldrs readme [OPTIONS] [PATH]
```

#### Options

| Option | Description | Example |
|--------|-------------|---------|
| `-p, --provider <PROVIDER>` | LLM provider (`gemini` or `openai`) | `--provider openai` |
| `-a, --api-key <API_KEY>` | API key for the selected provider | `--api-key sk-...` |
| `-s, --streaming` | Enable real-time streaming mode | `--streaming` |
| `--prompt <PROMPT>` | Custom prompt for generation | `--prompt "Focus on API docs"` |
| `--prompt-file <FILE>` | Load custom prompt from file | `--prompt-file prompts/api.txt` |
| `--instructions <TEXT>` | Additional instructions | `--instructions "Add benchmarks"` |
| `-h, --help` | Show help information | `--help` |

### Examples

#### Basic Generation
```bash
# Generate README with default settings (Gemini)
tldrs readme

# Use OpenAI instead
tldrs readme --provider openai
```

#### Streaming Mode
```bash
# Watch the README generate in real-time
tldrs readme --streaming
```

#### Custom Prompts
```bash
# Use a custom prompt directly
tldrs readme --prompt "Create a README focusing on installation and basic usage only"

# Load prompt from file
tldrs readme --prompt-file my-custom-prompt.txt

# Add extra instructions to default prompt
tldrs readme --instructions "Include performance benchmarks and add emoji for better readability"

# Combine custom prompt with additional instructions
tldrs readme --prompt-file technical-prompt.txt --instructions "Focus on enterprise deployment"
```

#### API Key Specification
```bash
# Pass API key directly (not recommended for scripts)
tldrs readme --api-key "your-api-key-here"

# Use with specific provider
tldrs readme --provider openai --api-key "sk-your-openai-key"
```

## 🔧 Configuration

### Environment Variables

tldrs uses environment variables for API key configuration:

```bash
# Google Gemini (Free tier available)
export GEMINI_API_KEY="your-gemini-api-key"

# OpenAI (Paid service)
export OPENAI_API_KEY="sk-your-openai-api-key"
```

### Getting API Keys

#### Google Gemini (Recommended)
1. Visit [Google AI Studio](https://aistudio.google.com)
2. Sign in with your Google account
3. Click "Get API Key" and create a new key
4. Free tier includes generous limits for personal projects

#### OpenAI
1. Visit [OpenAI Platform](https://platform.openai.com)
2. Sign up or log in to your account
3. Navigate to API Keys section
4. Create a new API key
5. Note: This is a paid service with usage-based billing

## 🎨 Custom Prompts

### Creating Custom Prompts

Create a text file with your custom prompt:

```text
# example-prompt.txt
You are a technical writer creating documentation for a software project.
Generate a README that includes:
1. A compelling project description
2. Clear installation instructions
3. Basic usage examples
4. API documentation if applicable
5. Contributing guidelines

Focus on clarity and include code examples where relevant.
Use a professional but approachable tone.
```

Use it with:
```bash
tldrs readme --prompt-file example-prompt.txt
```

### Prompt Best Practices

1. **Be Specific**: Clearly state what sections you want
2. **Set Tone**: Specify the writing style (professional, casual, technical)
3. **Include Examples**: Ask for code examples and practical usage
4. **Structure**: Define the README structure you prefer
5. **Audience**: Specify your target audience (developers, end-users, etc.)

## 🏗️ How It Works

1. **Code Analysis**: tldrs scans your project directory, analyzing:
   - Project structure and file organization
   - Configuration files (Cargo.toml, package.json, etc.)
   - Source code patterns and dependencies
   - Documentation and comments

2. **Content Extraction**: Intelligently filters and prioritizes:
   - Main application logic
   - Configuration and setup files
   - API definitions and interfaces
   - Build and dependency information

3. **AI Generation**: Sends structured context to your chosen LLM with:
   - System prompt for README generation
   - Your custom prompt (if provided)
   - Additional instructions
   - Extracted code context

4. **Output Creation**: Generates a professional README.md with:
   - Project overview and description
   - Installation instructions
   - Usage examples and documentation
   - Contributing guidelines
   - License information

## 🔍 Troubleshooting

### Common Issues

**"API key not valid" error**
```bash
# Verify your API key is set correctly
echo $GEMINI_API_KEY
# or
echo $OPENAI_API_KEY

# Make sure there are no extra spaces or quotes
export GEMINI_API_KEY="your-key-without-quotes"
```

**"Cannot specify both --prompt and --prompt-file" error**
```bash
# Use only one prompt option at a time
tldrs readme --prompt "your prompt"
# OR
tldrs readme --prompt-file prompt.txt
```

**Empty or poor quality README**
- Ensure your project has sufficient code content
- Try adding `--instructions` to guide the AI
- Use streaming mode (`--streaming`) to see generation progress
- Consider switching LLM providers

**Large project timeouts**
- Use streaming mode for better performance
- Ensure your API key has sufficient quota
- Consider breaking large monorepos into separate generations

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### Development Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/pixperk/tldrs.git
   cd tldrs
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Run locally**:
   ```bash
   cargo run -- readme --help
   ```

### Contributing Guidelines

- Fork the repository and create a feature branch
- Write tests for new functionality
- Ensure code follows Rust best practices
- Update documentation as needed
- Submit a pull request with a clear description

## 📊 Performance

- **Analysis Speed**: Processes most codebases in under 10 seconds
- **Generation Time**: Typically 15-45 seconds depending on LLM provider and project size
- **Memory Usage**: Minimal footprint, suitable for CI/CD environments
- **Supported Project Sizes**: From small scripts to large enterprise codebases

## 🗺️ Roadmap

- [ ] **Additional LLM Providers**: Claude, Llama, and local models
- [ ] **Template System**: Pre-built templates for different project types
- [ ] **Configuration Files**: Support for `.tldrs.toml` config files
- [ ] **Batch Processing**: Generate READMEs for multiple projects
- [ ] **Integration**: GitHub Actions and CI/CD pipeline integration
- [ ] **Interactive Mode**: Step-by-step README customization
- [ ] **Multi-language**: Support for non-English documentation

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ in Rust
- Powered by Google Gemini and OpenAI GPT models
- Inspired by the need for better, automated documentation

---

**Made with tldrs? Add this badge to your README:**

```markdown
[![Documentation generated with tldrs](https://img.shields.io/badge/docs-tldrs-blue)](https://github.com/pixperk/tldrs)
```

[![Documentation generated with tldrs](https://img.shields.io/badge/docs-tldrs-blue)](https://github.com/pixperk/tldrs)
