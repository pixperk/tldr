use std::path::Path;

/// Directories that should be completely skipped during codebase scanning
/// These typically contain build artifacts, dependencies, or auto-generated files
pub const SKIP_DIRECTORIES: [&str; 61] = [
    // Build outputs and artifacts
    "target", "build", "dist", "out", "bin", "obj", "Debug", "Release",
    ".build", "_build", "builds", "output", "artifacts",
    
    // Dependencies and package managers
    "node_modules", "vendor", "packages", ".yarn", ".pnp",
    "__pycache__", ".mypy_cache", ".pytest_cache", "site-packages",
    ".virtualenv", "venv", "env", ".env", "virtualenv",
    "Pods", "DerivedData", "xcuserdata", ".gradle",
    
    // Version control and IDE
    ".git", ".svn", ".hg", ".bzr", "_darcs",
    ".vscode", ".idea", ".vs", ".eclipse", ".sublime-text",
    
    // Temporary and cache files
    "tmp", "temp", ".tmp", ".cache", "cache", "logs", "log",
    ".DS_Store", "Thumbs.db", ".sass-cache", ".parcel-cache",
    
    // Documentation builds
    "_site", ".jekyll-cache", ".next", ".nuxt", ".docusaurus",
    
    // Test coverage and reports
    "coverage", "htmlcov", ".nyc_output", ".coverage",
];

/// File patterns that should be skipped
pub const SKIP_FILE_PATTERNS: [&str; 34] = [
    // Lock files and dependencies
    "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "Cargo.lock",
    "Pipfile.lock", "poetry.lock", "composer.lock", "Gemfile.lock",
    
    // Build and config artifacts
    "*.min.js", "*.bundle.js", "*.chunk.js", "*.map",
    "*.pyc", "*.pyo", "*.class", "*.jar", "*.war",
    "*.exe", "*.dll", "*.so", "*.dylib", "*.a", "*.lib",
    
    // IDE and editor files
    ".DS_Store", "Thumbs.db", "*.swp", "*.swo", "*~",
    "*.bak", "*.orig", "*.rej",
    
    // Log and temporary files
    "*.log", "*.tmp", "*.temp",
];

/// Important files that should always be included regardless of extension
pub const IMPORTANT_FILES: [&str; 20] = [
    // Configuration files
    "Cargo.toml", "package.json", "pyproject.toml", "setup.py", "requirements.txt",
    "Dockerfile", "docker-compose.yml", "docker-compose.yaml",
    "Makefile", "CMakeLists.txt", "build.gradle", "pom.xml",
    "go.mod", "go.sum", "composer.json",
    
    // Documentation and project files
    "README", "README.md", "README.txt", "LICENSE", "CHANGELOG.md",
];

/// Check if a directory should be skipped
pub fn should_skip_directory(dir_name: &str) -> bool {
    let dir_lower = dir_name.to_lowercase();
    
    // Check exact matches
    if SKIP_DIRECTORIES.iter().any(|&skip_dir| dir_lower == skip_dir.to_lowercase()) {
        return true;
    }
    
    // Check patterns
    if dir_lower.starts_with('.') && dir_lower.len() > 1 {
        // Skip most hidden directories except some important ones
        let important_hidden = [".github", ".gitlab"];
        if !important_hidden.iter().any(|&imp| dir_lower == imp) {
            return true;
        }
    }
    
    // Skip directories with common build/temp patterns
    if dir_lower.contains("cache") || 
       dir_lower.contains("temp") || 
       dir_lower.contains("tmp") ||
       dir_lower.ends_with("_modules") ||
       dir_lower.ends_with("_cache") {
        return true;
    }
    
    false
}

/// Check if a file should be skipped
pub fn should_skip_file(file_path: &Path) -> bool {
    let file_name = file_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    
    let file_name_lower = file_name.to_lowercase();
    
    // Always include important files
    if IMPORTANT_FILES.iter().any(|&imp| {
        file_name_lower == imp.to_lowercase() || 
        file_name.eq_ignore_ascii_case(imp)
    }) {
        return false;
    }
    
    // Check file patterns
    for pattern in &SKIP_FILE_PATTERNS {
        if pattern.contains('*') {
            // Simple wildcard matching
            let pattern_parts: Vec<&str> = pattern.split('*').collect();
            if pattern_parts.len() == 2 {
                let (prefix, suffix) = (pattern_parts[0], pattern_parts[1]);
                if file_name_lower.starts_with(prefix) && file_name_lower.ends_with(suffix) {
                    return true;
                }
            }
        } else if file_name_lower == pattern.to_lowercase() {
            return true;
        }
    }
    
    // Skip very large files (likely binary or generated)
    if let Ok(metadata) = file_path.metadata() {
        if metadata.len() > 1_000_000 { // 1MB threshold
            return true;
        }
    }
    
    // Skip files without extensions that are likely binaries
    if file_path.extension().is_none() {
        // Unless it's in our important files list or has specific patterns
        let is_script = file_name_lower.starts_with("makefile") ||
                       file_name_lower == "dockerfile" ||
                       file_name_lower == "rakefile" ||
                       file_name_lower == "gruntfile" ||
                       file_name_lower == "gulpfile";
        
        if !is_script {
            // Check if file might be executable/binary by trying to read first few bytes
            if let Ok(content) = std::fs::read(file_path) {
                if content.len() > 10 {
                    // Check for binary file signatures
                    let first_bytes = &content[..std::cmp::min(8, content.len())];
                    if first_bytes.contains(&0) { // Contains null bytes, likely binary
                        return true;
                    }
                }
            }
        }
    }
    
    false
}

/// Get priority score for a file (higher = more important)
pub fn get_file_priority(file_path: &Path) -> u32 {
    let file_name = file_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    
    let file_name_lower = file_name.to_lowercase();
    let path_str = file_path.to_string_lossy().to_lowercase();
    
    // High priority files
    if IMPORTANT_FILES.iter().any(|&imp| file_name.eq_ignore_ascii_case(imp)) {
        return 100;
    }
    
    // Source files in main/src directories
    if path_str.contains("/src/") || path_str.contains("\\src\\") ||
       path_str.contains("/main/") || path_str.contains("\\main\\") {
        return 80;
    }
    
    // Main application files
    if file_name_lower.contains("main") || 
       file_name_lower.contains("index") ||
       file_name_lower.contains("app") {
        return 70;
    }
    
    // Configuration and build files
    if file_name_lower.ends_with(".toml") ||
       file_name_lower.ends_with(".json") ||
       file_name_lower.ends_with(".yaml") ||
       file_name_lower.ends_with(".yml") {
        return 60;
    }
    
    // Source code files
    if file_path.extension().is_some() {
        return 50;
    }
    
    // Everything else
    20
}
